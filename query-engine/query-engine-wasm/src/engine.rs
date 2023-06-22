use crate::error::ApiError;
use futures::FutureExt;
use psl::PreviewFeature;
use query_core::{
    protocol::EngineProtocol,
    schema::{self, QuerySchema},
    telemetry, QueryExecutor, TransactionOptions, TxId,
};
use query_engine_metrics::{MetricFormat, MetricRegistry};
use request_handlers::{dmmf, load_executor, render_graphql_schema, RequestBody, RequestHandler};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    panic::AssertUnwindSafe,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::RwLock;
use user_facing_errors::Error;
use wasm_bindgen::prelude::*;

/// The main query engine used by JS
#[napi]
pub struct QueryEngine {
    inner: RwLock<Inner>,
}

/// The state of the engine.
enum Inner {
    /// Not connected, holding all data to form a connection.
    Builder(EngineBuilder),
    /// A connected engine, holding all data to disconnect and form a new
    /// connection. Allows querying when on this state.
    Connected(ConnectedEngine),
}

/// Everything needed to connect to the database and have the core running.
struct EngineBuilder {
    schema: Arc<psl::ValidatedSchema>,
    config_dir: PathBuf,
    env: HashMap<String, String>,
    engine_protocol: EngineProtocol,
}

/// Internal structure for querying and reconnecting with the engine.
struct ConnectedEngine {
    schema: Arc<psl::ValidatedSchema>,
    query_schema: Arc<QuerySchema>,
    executor: crate::Executor,
    config_dir: PathBuf,
    env: HashMap<String, String>,
    metrics: Option<MetricRegistry>,
    engine_protocol: EngineProtocol,
}

/// Returned from the `serverInfo` method in javascript.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerInfo {
    commit: String,
    version: String,
    primary_connector: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MetricOptions {
    format: MetricFormat,
    #[serde(default)]
    global_labels: HashMap<String, String>,
}

impl MetricOptions {
    fn is_json_format(&self) -> bool {
        self.format == MetricFormat::Json
    }
}

impl ConnectedEngine {
    /// The schema AST for Query Engine core.
    pub fn query_schema(&self) -> &Arc<QuerySchema> {
        &self.query_schema
    }

    /// The query executor.
    pub fn executor(&self) -> &(dyn QueryExecutor + Send + Sync) {
        self.executor.as_ref()
    }

    pub fn engine_protocol(&self) -> EngineProtocol {
        self.engine_protocol
    }
}

/// Parameters defining the construction of an engine.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConstructorOptions {
    datamodel: String,
    log_level: String,
    #[serde(default)]
    log_queries: bool,
    #[serde(default)]
    datasource_overrides: BTreeMap<String, String>,
    #[serde(default)]
    env: serde_json::Value,
    config_dir: PathBuf,
    #[serde(default)]
    ignore_env_var_errors: bool,
    #[serde(default)]
    engine_protocol: Option<EngineProtocol>,
}

impl Inner {
    /// Returns a builder if the engine is not connected
    fn as_builder(&self) -> crate::Result<&EngineBuilder> {
        match self {
            Inner::Builder(ref builder) => Ok(builder),
            Inner::Connected(_) => Err(ApiError::AlreadyConnected),
        }
    }

    /// Returns the engine if connected
    fn as_engine(&self) -> crate::Result<&ConnectedEngine> {
        match self {
            Inner::Builder(_) => Err(ApiError::NotConnected),
            Inner::Connected(ref engine) => Ok(engine),
        }
    }
}

#[wasm_bindgen]
impl QueryEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(
        napi_env: Env,
        options: ConstructorOptions,
        // maybe_driver: Option<JsObject>,
    ) -> Self {
        // #[cfg(feature = "js-drivers")]
        // if let Some(driver) = maybe_driver {
        //     let queryable = js_drivers::JsQueryable::from(driver);
        //     sql_connector::register_driver(Arc::new(queryable));
        // }

        let ConstructorOptions {
            datamodel,
            log_level,
            log_queries,
            datasource_overrides,
            env,
            config_dir,
            ignore_env_var_errors,
            engine_protocol,
        } = options;

        let env = stringify_env_values(env).unwrap(); // we cannot trust anything JS sends us from process.env
        let overrides: Vec<(_, _)> = datasource_overrides.into_iter().collect();
        let mut schema = psl::validate(datamodel.into());
        let config = &mut schema.configuration;

        schema
            .diagnostics
            .to_result()
            .map_err(|err| ApiError::conversion(err, schema.db.source()))
            .unwrap();

        config
            .resolve_datasource_urls_query_engine(
                &overrides,
                |key| env.get(key).map(ToString::to_string),
                ignore_env_var_errors,
            )
            .map_err(|err| ApiError::conversion(err, schema.db.source()))
            .unwrap();

        config
            .validate_that_one_datasource_is_provided()
            .map_err(|errors| ApiError::conversion(errors, schema.db.source()))
            .unwrap();

        let engine_protocol = engine_protocol.unwrap_or(EngineProtocol::Json);

        let builder = EngineBuilder {
            schema: Arc::new(schema),
            config_dir,
            engine_protocol,
            env,
        };

        Ok(Self {
            inner: RwLock::new(Inner::Builder(builder)),
        })
    }

    /// Connect to the database, allow queries to be run.
    #[wasm_bindgen]
    pub async fn connect(&self, trace: String) -> Result<(), JsError> {
        let mut inner = self.inner.write().await;
        let builder = inner.as_builder()?;
        let arced_schema = Arc::clone(&builder.schema);
        let arced_schema_2 = Arc::clone(&builder.schema);

        let url = {
            let data_source = builder
                .schema
                .configuration
                .datasources
                .first()
                .ok_or_else(|| ApiError::configuration("No valid data source found"))?;
            data_source
                .load_url_with_config_dir(&builder.config_dir, |key| builder.env.get(key).map(ToString::to_string))
                .map_err(|err| crate::error::ApiError::Conversion(err, builder.schema.db.source().to_owned()))?
        };

        let engine = async move {
            // We only support one data source & generator at the moment, so take the first one (default not exposed yet).
            let data_source = arced_schema
                .configuration
                .datasources
                .first()
                .ok_or_else(|| ApiError::configuration("No valid data source found"))?;

            let preview_features = arced_schema.configuration.preview_features();

            let executor_fut = async {
                let executor = load_executor(data_source, preview_features, &url).await?;
                let connector = executor.primary_connector();

                let conn_span = tracing::info_span!(
                    "prisma:engine:connection",
                    user_facing = true,
                    "db.type" = connector.name(),
                );

                connector.get_connection().instrument(conn_span).await?;

                crate::Result::<_>::Ok(executor)
            };

            let query_schema_span = tracing::info_span!("prisma:engine:schema");
            let query_schema_fut = tokio::runtime::Handle::current().spawn_blocking(move || {
                let enable_raw_queries = true;
                schema::build(arced_schema_2, enable_raw_queries)
            });

            let (query_schema, executor) = tokio::join!(query_schema_fut, executor_fut);

            Ok(ConnectedEngine {
                schema: builder.schema.clone(),
                query_schema: Arc::new(query_schema.unwrap()),
                executor: executor?,
                config_dir: builder.config_dir.clone(),
                env: builder.env.clone(),
                metrics: None,
                engine_protocol: builder.engine_protocol,
            }) as crate::Result<ConnectedEngine>
        }
        .await?;

        *inner = Inner::Connected(engine);

        Ok(())
    }

    /// Disconnect and drop the core. Can be reconnected later with `#connect`.
    #[wasm_bindgen]
    pub async fn disconnect(&self, trace: String) -> napi::Result<()> {
        // TODO: when using Node Drivers, we need to call Driver::close() here.
        let mut inner = self.inner.write().await;
        let engine = inner.as_engine()?;

        let builder = EngineBuilder {
            schema: engine.schema.clone(),
            config_dir: engine.config_dir.clone(),
            env: engine.env.clone(),
            engine_protocol: engine.engine_protocol(),
        };

        *inner = Inner::Builder(builder);

        Ok(())
    }

    /// If connected, sends a query to the core and returns the response.
    #[wasm_bindgen]
    pub async fn query(&self, body: String, trace: String, tx_id: Option<String>) -> Result<String, JsError> {
        let inner = self.inner.read().await;
        let engine = inner.as_engine()?;

        let query = RequestBody::try_from_str(&body, engine.engine_protocol())?;
        let handler = RequestHandler::new(engine.executor(), engine.query_schema(), engine.engine_protocol());
        let response = handler.handle(query, tx_id.map(TxId::from), tx_id).await;

        Ok(serde_json::to_string(&response)?)
    }

    /// If connected, attempts to start a transaction in the core and returns its ID.
    #[wasm_bindgen(js_name = startTransaction)]
    pub async fn start_transaction(&self, input: String, trace: String) -> Result<String, JsError> {
        let inner = self.inner.read().await;
        let engine = inner.as_engine()?;

        let tx_opts: TransactionOptions = serde_json::from_str(&input)?;
        match engine
            .executor()
            .start_tx(engine.query_schema().clone(), engine.engine_protocol(), tx_opts)
            .await
        {
            Ok(tx_id) => Ok(json!({ "id": tx_id.to_string() }).to_string()),
            Err(err) => Ok(map_known_error(err)?),
        }
    }

    /// If connected, attempts to commit a transaction with id `tx_id` in the core.
    #[wasm_bindgen(js_name = commitTransaction)]
    pub async fn commit_transaction(&self, tx_id: String, _trace: String) -> Result<String, JsError> {
        let inner = self.inner.read().await;
        let engine = inner.as_engine()?;

        match engine.executor().commit_tx(TxId::from(tx_id)).await {
            Ok(_) => Ok("{}".to_string()),
            Err(err) => Ok(map_known_error(err)?),
        }
    }

    #[wasm_bindgen]
    pub async fn dmmf(&self, trace: String) -> Result<String, JsError> {
        let inner = self.inner.read().await;
        let engine = inner.as_engine()?;

        let dmmf = dmmf::render_dmmf(&engine.query_schema);
        let json = serde_json::to_string(&dmmf).unwrap();

        Ok(json)
    }

    /// If connected, attempts to roll back a transaction with id `tx_id` in the core.
    #[wasm_bindgen(js_name = rollbackTransaction)]
    pub async fn rollback_transaction(&self, tx_id: String, _trace: String) -> Result<String, JsError> {
        let inner = self.inner.read().await;
        let engine = inner.as_engine()?;

        async move {
            match engine.executor().rollback_tx(TxId::from(tx_id)).await {
                Ok(_) => Ok("{}".to_string()),
                Err(err) => Ok(map_known_error(err)?),
            }
        }
    }
}

fn map_known_error(err: query_core::CoreError) -> crate::Result<String> {
    let user_error: user_facing_errors::Error = err.into();
    let value = serde_json::to_string(&user_error)?;

    Ok(value)
}

fn stringify_env_values(origin: serde_json::Value) -> crate::Result<HashMap<String, String>> {
    use serde_json::Value;

    let msg = match origin {
        Value::Object(map) => {
            let mut result: HashMap<String, String> = HashMap::new();

            for (key, val) in map.into_iter() {
                match val {
                    Value::Null => continue,
                    Value::String(val) => {
                        result.insert(key, val);
                    }
                    val => {
                        result.insert(key, val.to_string());
                    }
                }
            }

            return Ok(result);
        }
        Value::Null => return Ok(Default::default()),
        Value::Bool(_) => "Expected an object for the env constructor parameter, got a boolean.",
        Value::Number(_) => "Expected an object for the env constructor parameter, got a number.",
        Value::String(_) => "Expected an object for the env constructor parameter, got a string.",
        Value::Array(_) => "Expected an object for the env constructor parameter, got an array.",
    };

    Err(ApiError::JsonDecode(msg.to_string()))
}
