use bigdecimal::{BigDecimal, FromPrimitive};
use indexmap::IndexMap;
use query_core::{Operation, QueryValue, Selection};
use schema::{InputField, InputType, ObjectType, OutputField, OutputFieldRef, QuerySchema, QueryTag, ScalarType};
use serde_json::Value;

use crate::HandlerError;

use super::query::{JsonQuery, JsonQueryField, SelectionOption, TopLevelActionKind};

pub fn convert_json_query_to_operation(query: JsonQuery, schema: &QuerySchema) -> crate::Result<Operation> {
    let kind = query.action.kind();

    match kind {
        TopLevelActionKind::Read => build_read_operation(query, &schema),
        TopLevelActionKind::Write => build_write_operation(query, &schema),
    }
}

fn build_read_operation(query: JsonQuery, schema: &QuerySchema) -> crate::Result<Operation> {
    let tag: QueryTag = query.action.into();

    let field = find_field_by_model_and_tag(&schema.query(), &query.model_name, tag)
        .ok_or_else(|| HandlerError::query_conversion("No passing top level query field found"))?;

    let selection = build_selection(&field.name, &field, &query.query)?;
    return Ok(Operation::Read(selection));
}

fn build_write_operation(query: JsonQuery, schema: &QuerySchema) -> crate::Result<Operation> {
    let tag: QueryTag = query.action.into();

    let field = find_field_by_model_and_tag(&schema.mutation(), &query.model_name, tag)
        .ok_or_else(|| HandlerError::query_conversion("No passing top level query field found"))?;

    let selection = build_selection(&field.name, &field, &query.query)?;
    return Ok(Operation::Write(selection));
}

fn find_field_by_model_and_tag(object_type: &ObjectType, model_name: &str, tag: QueryTag) -> Option<OutputFieldRef> {
    object_type
        .get_fields()
        .into_iter()
        .find(|field| {
            if let Some(query_info) = &field.query_info {
                if let Some(model) = &query_info.model {
                    return model.name == model_name && query_info.tag == tag;
                }
            }

            false
        })
        .cloned()
}

fn build_selection(name: &str, field: &OutputField, query_field: &JsonQueryField) -> crate::Result<Selection> {
    let mut selection = Selection::with_name(name);
    if query_field.selection.select.is_some() && query_field.selection.include.is_some() {
        return Err(HandlerError::query_conversion(
            "Both 'select' and 'include' are set on the same field",
        ));
    }
    let object_type = field
        .field_type
        .as_object_type()
        .ok_or_else(|| HandlerError::query_conversion("Expected object type"))?;

    if let Some(select) = &query_field.selection.select {
        add_selection_fields(&mut selection, &object_type, select)?;
    } else {
        add_default_selections(&mut selection, &object_type)?;
        if let Some(include) = &query_field.selection.include {
            add_selection_fields(&mut selection, &object_type, include)?;
        }
    }

    add_arguments(&mut selection, &field, &query_field)?;

    return Ok(selection);
}

fn add_selection_fields(
    selection: &mut Selection,
    field_type: &ObjectType,
    fields_map: &IndexMap<String, SelectionOption>,
) -> crate::Result<()> {
    for (name, option) in fields_map.iter() {
        let sub_field = field_type
            .find_field(name)
            .ok_or_else(|| HandlerError::query_conversion(format!("Can't find field {}", name)))?;

        match option {
            SelectionOption::Bool(include_field) => {
                if *include_field {
                    selection.push_nested_selection(build_default_selection(&sub_field)?)
                }
            }

            SelectionOption::SubQuery(query_field) => {
                selection.push_nested_selection(build_selection(&name, &sub_field, query_field)?)
            }
        }
    }

    Ok(())
}

fn add_default_selections(selection: &mut Selection, field_type: &ObjectType) -> crate::Result<()> {
    field_type
        .get_fields()
        .into_iter()
        .filter(|sub_field| sub_field.field_type.is_scalar())
        .map(|sub_field| Selection::with_name(&sub_field.name))
        .for_each(|nested_selection| selection.push_nested_selection(nested_selection));
    Ok(())
}

fn build_default_selection(field: &OutputField) -> crate::Result<Selection> {
    let mut selection = Selection::with_name(&field.name);
    if let Some(object_type) = field.field_type.as_object_type() {
        add_default_selections(&mut selection, &object_type)?;
    }
    Ok(selection)
}

fn add_arguments(selection: &mut Selection, field: &OutputField, query_field: &JsonQueryField) -> crate::Result<()> {
    for (key, value) in &query_field.arguments {
        let input_field = field
            .arguments
            .iter()
            .find(|arg| arg.name == *key)
            .ok_or_else(|| HandlerError::query_conversion(format!("Unknown argument: {}", key)))?;

        selection.push_argument(key, build_argument_from_field(input_field, value)?)
    }

    Ok(())
}

fn build_argument_from_field(field: &InputField, value: &Value) -> crate::Result<QueryValue> {
    if value.is_null() {
        if field.is_required {
            return Err(HandlerError::query_conversion(format!(
                "field {} is required",
                field.name
            )));
        }
    }

    field
        .field_types
        .iter()
        .find_map(|field_type| build_argument_from_type(field_type, value).ok())
        .ok_or_else(|| HandlerError::query_conversion("Value does not match any input type"))
}

fn build_argument_from_type(input_type: &InputType, value: &Value) -> crate::Result<QueryValue> {
    match input_type {
        InputType::Scalar(ScalarType::Null) => value
            .as_null()
            .map(|()| QueryValue::Null)
            .ok_or_else(|| HandlerError::query_conversion("null value expected")),
        InputType::Scalar(ScalarType::Int) => value
            .as_i64()
            .map(QueryValue::Int)
            .ok_or_else(|| HandlerError::query_conversion(format!("Invalid value for integer field {:}", value))),
        InputType::Scalar(ScalarType::Float) => value
            .as_f64()
            .and_then(BigDecimal::from_f64)
            .map(QueryValue::Float)
            .ok_or_else(|| HandlerError::query_conversion(format!("Invalid value for float field {:}", value))),
        InputType::Scalar(ScalarType::Boolean) => value
            .as_bool()
            .map(QueryValue::Boolean)
            .ok_or_else(|| HandlerError::query_conversion(format!("Invalid value for bool field {:}", value))),
        InputType::Scalar(
            ScalarType::String
            | ScalarType::UUID
            | ScalarType::Bytes
            | ScalarType::DateTime
            | ScalarType::Decimal
            | ScalarType::BigInt
            | ScalarType::Xml,
        ) => value
            .as_str()
            .map(|s| QueryValue::String(s.into()))
            .ok_or_else(|| HandlerError::query_conversion(format!("Invalid value for string-like field {:}", value))),

        InputType::Scalar(ScalarType::Json | ScalarType::JsonList) => {
            Ok(serde_json::to_string(&value).map(QueryValue::String).unwrap())
        }

        InputType::Enum(_) => value
            .as_str()
            .map(|str| QueryValue::Enum(str.into()))
            .ok_or_else(|| HandlerError::query_conversion(format!("Invalid value for enum field {:}", value))),

        InputType::List(inner_type) => value
            .as_array()
            .ok_or_else(|| HandlerError::query_conversion("Not an array"))?
            .iter()
            .map(|value| build_argument_from_type(&inner_type, value))
            .collect::<crate::Result<Vec<QueryValue>>>()
            .map(QueryValue::List),

        InputType::Object(object_type) => {
            let object_type = object_type.upgrade().unwrap();

            value
                .as_object()
                .ok_or_else(|| HandlerError::query_conversion("Not an object"))?
                .into_iter()
                .map(|(key, value)| {
                    object_type
                        .find_field(key)
                        .ok_or_else(|| HandlerError::query_conversion("field not found"))
                        .and_then(|field| build_argument_from_field(&field, value))
                        .map(|arg| (key.to_owned(), arg))
                })
                .collect::<crate::Result<IndexMap<String, QueryValue>>>()
                .map(QueryValue::Object)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use std::sync::Arc;

    fn schema() -> schema::QuerySchema {
        let schema_str = r#"
        generator client {
            provider        = "prisma-client-js"
          }
          
          datasource db {
            provider = "sqlite"
            url      = "file:./dev.db"
          }
          
          model User {
            id String @id
            name String?
            email String @unique
            posts Post[]
          }

          model Post {
            id String @id
            title String
            userId String 
            user User @relation(fields: [userId], references: [id])
          }
        "#;
        let mut schema = psl::validate(schema_str.into());

        schema.diagnostics.to_result().unwrap();

        let internal_data_model = prisma_models::convert(Arc::new(schema), "".into());
        schema_builder::build(internal_data_model, true)
    }

    #[test]
    pub fn default_selection() {
        let query: JsonQuery = serde_json::from_str(
            &r#"{
            "modelName": "User",
            "action": "findFirst",
            "query": {}
        }"#,
        )
        .unwrap();

        let operation = convert_json_query_to_operation(query, &schema()).unwrap();

        assert_debug_snapshot!(operation, @r###"
        Read(
            Selection {
                name: "findFirstUser",
                alias: None,
                arguments: [],
                nested_selections: [
                    Selection {
                        name: "id",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "name",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "email",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                ],
            },
        )
        "###);
    }

    #[test]
    pub fn explicit_select() {
        let query: JsonQuery = serde_json::from_str(
            &r#"{
            "modelName": "User",
            "action": "findFirst",
            "select": {
                "id": true,
                "email": false
            }
        }"#,
        )
        .unwrap();

        let operation = convert_json_query_to_operation(query, &schema()).unwrap();

        assert_debug_snapshot!(operation, @r###"
        Read(
            Selection {
                name: "findFirstUser",
                alias: None,
                arguments: [],
                nested_selections: [
                    Selection {
                        name: "id",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                ],
            },
        )
        "###);
    }

    #[test]
    pub fn arguments() {
        let query: JsonQuery = serde_json::from_str(
            &r#"{
            "modelName": "User",
            "action": "findFirst",
            "where": {
                "id": "123"
            }
        }"#,
        )
        .unwrap();

        let operation = convert_json_query_to_operation(query, &schema()).unwrap();

        assert_debug_snapshot!(operation, @r###"
        Read(
            Selection {
                name: "findFirstUser",
                alias: None,
                arguments: [
                    (
                        "where",
                        Object(
                            {
                                "id": String(
                                    "123",
                                ),
                            },
                        ),
                    ),
                ],
                nested_selections: [
                    Selection {
                        name: "id",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "name",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "email",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                ],
            },
        )
        "###);
    }

    #[test]
    pub fn nested_arguments() {
        let query: JsonQuery = serde_json::from_str(
            &r#"{
            "modelName": "User",
            "action": "findFirst",
            "include": {
                "posts": {
                    "where": { "title": "something" }
                }
            }
        }"#,
        )
        .unwrap();

        let operation = convert_json_query_to_operation(query, &schema()).unwrap();

        assert_debug_snapshot!(operation, @r###"
        Read(
            Selection {
                name: "findFirstUser",
                alias: None,
                arguments: [],
                nested_selections: [
                    Selection {
                        name: "id",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "name",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "email",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "posts",
                        alias: None,
                        arguments: [
                            (
                                "where",
                                Object(
                                    {
                                        "title": String(
                                            "something",
                                        ),
                                    },
                                ),
                            ),
                        ],
                        nested_selections: [
                            Selection {
                                name: "id",
                                alias: None,
                                arguments: [],
                                nested_selections: [],
                            },
                            Selection {
                                name: "title",
                                alias: None,
                                arguments: [],
                                nested_selections: [],
                            },
                            Selection {
                                name: "userId",
                                alias: None,
                                arguments: [],
                                nested_selections: [],
                            },
                        ],
                    },
                ],
            },
        )
        "###);
    }

    #[test]
    pub fn include_true() {
        let query: JsonQuery = serde_json::from_str(
            &r#"{
            "modelName": "User",
            "action": "findFirst",
            "include": {
                "posts": true
            }
        }"#,
        )
        .unwrap();

        let operation = convert_json_query_to_operation(query, &schema()).unwrap();

        assert_debug_snapshot!(operation, @r###"
        Read(
            Selection {
                name: "findFirstUser",
                alias: None,
                arguments: [],
                nested_selections: [
                    Selection {
                        name: "id",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "name",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "email",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                    Selection {
                        name: "posts",
                        alias: None,
                        arguments: [],
                        nested_selections: [
                            Selection {
                                name: "id",
                                alias: None,
                                arguments: [],
                                nested_selections: [],
                            },
                            Selection {
                                name: "title",
                                alias: None,
                                arguments: [],
                                nested_selections: [],
                            },
                            Selection {
                                name: "userId",
                                alias: None,
                                arguments: [],
                                nested_selections: [],
                            },
                        ],
                    },
                ],
            },
        )
        "###);
    }

    #[test]
    pub fn mutation() {
        let query: JsonQuery = serde_json::from_str(
            &r#"{
            "modelName": "User",
            "action": "create",
            "select": {
                "id": true,
                "email": false
            }
        }"#,
        )
        .unwrap();

        let operation = convert_json_query_to_operation(query, &schema()).unwrap();

        assert_debug_snapshot!(operation, @r###"
        Write(
            Selection {
                name: "createOneUser",
                alias: None,
                arguments: [],
                nested_selections: [
                    Selection {
                        name: "id",
                        alias: None,
                        arguments: [],
                        nested_selections: [],
                    },
                ],
            },
        )
        "###);
    }
}
