use indexmap::IndexMap;
use schema::QueryTag;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsonQuery {
    pub model_name: String,
    pub action: TopLevelAction,

    #[serde(flatten)]
    pub query: JsonQueryField,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TopLevelAction {
    FindUnique,
    FindUniqueOrThrow,
    FindFirst,
    FindFirstOrThrow,
    FindMany,
    Create,
    CreateMany,
    Update,
    UpdateMany,
    Delete,
    DeleteMany,
    Upsert,
    Aggregate,
    GroupBy,
    ExecuteRaw,
    QueryRaw,
}

pub enum TopLevelActionKind {
    Read,
    Write,
}

impl TopLevelAction {
    pub fn kind(&self) -> TopLevelActionKind {
        match self {
            Self::FindUnique
            | Self::FindUniqueOrThrow
            | Self::FindFirst
            | Self::FindFirstOrThrow
            | Self::FindMany
            | Self::Aggregate
            | Self::GroupBy
            | Self::QueryRaw => TopLevelActionKind::Read,

            Self::Create
            | Self::CreateMany
            | Self::Update
            | Self::UpdateMany
            | Self::Delete
            | Self::DeleteMany
            | Self::Upsert
            | Self::ExecuteRaw => TopLevelActionKind::Write,
        }
    }
}

impl From<TopLevelAction> for QueryTag {
    fn from(action: TopLevelAction) -> Self {
        match action {
            TopLevelAction::FindUnique => Self::FindUnique,
            TopLevelAction::FindUniqueOrThrow => Self::FindUniqueOrThrow,
            TopLevelAction::FindFirst => Self::FindFirst,
            TopLevelAction::FindFirstOrThrow => Self::FindFirstOrThrow,
            TopLevelAction::FindMany => Self::FindMany,
            TopLevelAction::Aggregate => Self::Aggregate,
            TopLevelAction::GroupBy => Self::GroupBy,
            // TODO: fill in query type
            TopLevelAction::QueryRaw => Self::QueryRaw { query_type: None },
            TopLevelAction::Create => Self::CreateOne,
            TopLevelAction::CreateMany => Self::CreateMany,
            TopLevelAction::Update => Self::UpdateOne,
            TopLevelAction::UpdateMany => Self::UpdateMany,
            TopLevelAction::Delete => Self::DeleteOne,
            TopLevelAction::DeleteMany => Self::DeleteMany,
            TopLevelAction::Upsert => Self::UpsertOne,
            TopLevelAction::ExecuteRaw => Self::ExecuteRaw,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonQueryField {
    #[serde(flatten)]
    pub selection: Selection,

    #[serde(flatten)]
    pub arguments: IndexMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Selection {
    pub select: Option<IndexMap<String, SelectionOption>>,
    pub include: Option<IndexMap<String, SelectionOption>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SelectionOption {
    Bool(bool),
    SubQuery(JsonQueryField),
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    pub fn basic_test() {
        let query: JsonQuery = serde_json::from_str(
            r#"{
              "modelName": "User",
              "action": "findMany",
              "where": {
                "id": 123
              },
              "include": {
                "posts": {
                    "where": { "deleted": false },
                    "select": {
                        "title": true,
                        "readCount": true
                    }
                }
              }
            }
        "#,
        )
        .unwrap();

        assert_debug_snapshot!(query, @r###"
        JsonQuery {
            model_name: "User",
            action: FindMany,
            query: JsonQueryField {
                selection: Selection {
                    select: None,
                    include: Some(
                        {
                            "posts": SubQuery(
                                JsonQueryField {
                                    selection: Selection {
                                        select: Some(
                                            {
                                                "title": Bool(
                                                    true,
                                                ),
                                                "readCount": Bool(
                                                    true,
                                                ),
                                            },
                                        ),
                                        include: None,
                                    },
                                    arguments: {
                                        "where": Object {
                                            "deleted": Bool(false),
                                        },
                                    },
                                },
                            ),
                        },
                    ),
                },
                arguments: {
                    "where": Object {
                        "id": Number(123),
                    },
                },
            },
        }
        "###)
    }
}
