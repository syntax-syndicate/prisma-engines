pub use crate::types::{ColumnType, JSResultSet, Query, TransactionOptions};
use crate::{from_js, get_named_property, to_rust_str, JsObject, JsResult, JsString};

use super::async_js_function::AsyncJsFunction;
use super::transaction::JsTransaction;
use metrics::increment_gauge;
use std::sync::atomic::{AtomicBool, Ordering};

/// Proxy is a struct wrapping a javascript object that exhibits basic primitives for
/// querying and executing SQL (i.e. a client connector). The Proxy uses NAPI ThreadSafeFunction to
/// invoke the code within the node runtime that implements the client connector.
pub(crate) struct CommonProxy {
    /// Execute a query given as SQL, interpolating the given parameters.
    query_raw: AsyncJsFunction<Query, JSResultSet>,

    /// Execute a query given as SQL, interpolating the given parameters and
    /// returning the number of affected rows.
    execute_raw: AsyncJsFunction<Query, u32>,

    /// Return the flavour for this driver.
    pub(crate) flavour: String,
}

/// This is a JS proxy for accessing the methods specific to top level
/// JS driver objects
pub(crate) struct DriverProxy {
    start_transaction: AsyncJsFunction<(), JsTransaction>,
}
/// This a JS proxy for accessing the methods, specific
/// to JS transaction objects
pub(crate) struct TransactionProxy {
    /// transaction options
    options: TransactionOptions,

    /// commit transaction
    commit: AsyncJsFunction<(), ()>,

    /// rollback transaction
    rollback: AsyncJsFunction<(), ()>,

    /// whether the transaction has already been committed or rolled back
    closed: AtomicBool,
}

impl CommonProxy {
    pub fn new(object: &JsObject) -> JsResult<Self> {
        let flavour: JsString = get_named_property(object, "flavour")?;

        Ok(Self {
            query_raw: get_named_property(object, "queryRaw")?,
            execute_raw: get_named_property(object, "executeRaw")?,
            flavour: to_rust_str(flavour)?,
        })
    }

    pub async fn query_raw(&self, params: Query) -> quaint::Result<JSResultSet> {
        self.query_raw.call(params).await
    }

    pub async fn execute_raw(&self, params: Query) -> quaint::Result<u32> {
        self.execute_raw.call(params).await
    }
}

impl DriverProxy {
    pub fn new(driver_adapter: &JsObject) -> JsResult<Self> {
        Ok(Self {
            start_transaction: get_named_property(driver_adapter, "startTransaction")?,
        })
    }

    pub async fn start_transaction(&self) -> quaint::Result<Box<JsTransaction>> {
        let tx = self.start_transaction.call(()).await?;

        // Decrement for this gauge is done in JsTransaction::commit/JsTransaction::rollback
        // Previously, it was done in JsTransaction::new, similar to the native Transaction.
        // However, correct Dispatcher is lost there and increment does not register, so we moved
        // it here instead.
        increment_gauge!("prisma_client_queries_active", 1.0);
        Ok(Box::new(tx))
    }
}

impl TransactionProxy {
    pub fn new(js_transaction: &JsObject) -> JsResult<Self> {
        let commit = get_named_property(js_transaction, "commit")?;
        let rollback = get_named_property(js_transaction, "rollback")?;
        let options = get_named_property(js_transaction, "options")?;
        let options = from_js::<TransactionOptions>(options);

        Ok(Self {
            commit,
            rollback,
            options,
            closed: AtomicBool::new(false),
        })
    }

    pub fn options(&self) -> &TransactionOptions {
        &self.options
    }

    /// Commits the transaction via the driver adapter.
    ///
    /// ## Cancellation safety
    ///
    /// The future is cancellation-safe as long as the underlying Node-API call
    /// is cancellation-safe and no new await points are introduced between storing true in
    /// [`TransactionProxy::closed`] and calling the underlying JS function.
    ///
    /// - If `commit` is called but never polled or awaited, it's a no-op, the transaction won't be
    ///   committed and [`TransactionProxy::closed`] will not be changed.
    ///
    /// - If it is polled at least once, `true` will be stored in [`TransactionProxy::closed`] and
    ///   the underlying FFI call will be delivered to JavaScript side in lockstep, so the destructor
    ///   will not attempt rolling the transaction back even if the `commit` future was dropped while
    ///   waiting on the JavaScript call to complete and deliver response.
    pub async fn commit(&self) -> quaint::Result<()> {
        self.closed.store(true, Ordering::Relaxed);
        self.commit.call(()).await
    }

    /// Rolls back the transaction via the driver adapter.
    ///
    /// ## Cancellation safety
    ///
    /// The future is cancellation-safe as long as the underlying Node-API call
    /// is cancellation-safe and no new await points are introduced between storing true in
    /// [`TransactionProxy::closed`] and calling the underlying JS function.
    ///
    /// - If `rollback` is called but never polled or awaited, it's a no-op, the transaction won't be
    ///   rolled back yet and [`TransactionProxy::closed`] will not be changed.
    ///
    /// - If it is polled at least once, `true` will be stored in [`TransactionProxy::closed`] and
    ///   the underlying FFI call will be delivered to JavaScript side in lockstep, so the destructor
    ///   will not attempt rolling back again even if the `rollback` future was dropped while waiting
    ///   on the JavaScript call to complete and deliver response.
    pub async fn rollback(&self) -> quaint::Result<()> {
        self.closed.store(true, Ordering::Relaxed);
        self.rollback.call(()).await
    }
}

impl Drop for TransactionProxy {
    fn drop(&mut self) {
        if self.closed.swap(true, Ordering::Relaxed) {
            return;
        }

        _ = self.rollback.call_non_blocking(());
    }
}
