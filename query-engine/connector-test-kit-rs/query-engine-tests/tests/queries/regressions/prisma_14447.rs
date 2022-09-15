//! https://github.com/prisma/prisma/issues/14447

use query_engine_tests::*;

#[test_suite(schema(schema))]
mod prisma_14447 {
    use indoc::indoc;

    fn schema() -> String {
        let s = indoc! {r#"
            model Group {
              id    Int    @id @default(autoincrement())
              name  String
              slots Slot[]
            }

            model Slot {
              id         Int       @id @default(autoincrement())
              from       DateTime
              groupId    Int
              group      Group     @relation(fields: [groupId], references: [id])
              capacityId Int?
              capacity   Capacity? @relation(fields: [capacityId], references: [id])
            }

            model Capacity {
              id    Int    @id @default(autoincrement())
              count Int
              slots Slot[]
            }
        "#};

        s.into()
    }

    #[connector_test]
    async fn single_nesting(runner: Runner) -> TestResult<()> {
        let query = indoc! {r#"
            mutation {
              createOneGroup(data: {
                name: "Group with capacity"
                slots: {
                  create: [
                    {
                      from: "2022-07-22T00:00:00.000Z"
                    },
                    {
                      from: "2022-07-23T00:00:00.000Z"
                    },
                    {
                      from: "2022-07-24T00:00:00.000Z"
                    }
                  ]
                }
              }) {
                id
                name
                slots {
                  id
                  from
                  groupId
                  capacityId
                }
              }
            }
        "#};

        let result = run_query!(&runner, query);
        let result =
            serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&result).unwrap()).unwrap();

        insta::assert_snapshot!(result, @r###"
        {
          "data": {
            "createOneGroup": {
              "id": 1,
              "name": "Group with capacity",
              "slots": [
                {
                  "id": 1,
                  "from": "2022-07-22T00:00:00.000Z",
                  "groupId": 1,
                  "capacityId": null
                },
                {
                  "id": 2,
                  "from": "2022-07-23T00:00:00.000Z",
                  "groupId": 1,
                  "capacityId": null
                },
                {
                  "id": 3,
                  "from": "2022-07-24T00:00:00.000Z",
                  "groupId": 1,
                  "capacityId": null
                }
              ]
            }
          }
        }
        "###);

        Ok(())
    }

    #[connector_test]
    async fn prisma_14447_double_nesting(runner: Runner) -> TestResult<()> {
        let query = indoc! {r#"
            mutation {
              createOneGroup(data: {
                name: "Group with capacity"
                slots: {
                  create: [
                    {
                      from: "2022-07-22T00:00:00.000Z"
                      capacity: {
                        create: {
                          count: 10
                        }
                      }
                    },
                    {
                      from: "2022-07-23T00:00:00.000Z"
                      capacity: {
                        create: {
                          count: 10
                        }
                      }
                    },
                    {
                      from: "2022-07-24T00:00:00.000Z"
                      capacity: {
                        create: {
                          count: 10
                        }
                      }
                    }
                  ]
                }
              }) {
                id
                name
                slots {
                  id
                  from
                  groupId
                  capacityId
                }
              }
            }
        "#};

        let result = run_query!(&runner, query);
        let result =
            serde_json::to_string_pretty(&serde_json::from_str::<serde_json::Value>(&result).unwrap()).unwrap();

        insta::assert_snapshot!(result, @r###"
        {
          "data": {
            "createOneGroup": {
              "id": 1,
              "name": "Group with capacity",
              "slots": [
                {
                  "id": 1,
                  "from": "2022-07-22T00:00:00.000Z",
                  "groupId": 1,
                  "capacityId": 1
                },
                {
                  "id": 2,
                  "from": "2022-07-23T00:00:00.000Z",
                  "groupId": 1,
                  "capacityId": 2
                },
                {
                  "id": 3,
                  "from": "2022-07-24T00:00:00.000Z",
                  "groupId": 1,
                  "capacityId": 3
                }
              ]
            }
          }
        }
        "###);

        Ok(())
    }
}
