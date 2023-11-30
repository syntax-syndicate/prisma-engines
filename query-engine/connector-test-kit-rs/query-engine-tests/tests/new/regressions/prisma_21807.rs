use query_engine_tests::*;

#[test_suite(schema(schema), only(Postgres))]
mod citext {
    fn schema() -> String {
        indoc! {
            r#"
            // generator client {
            //   provider        = "prisma-client-js"
            //   previewFeatures = ["postgresqlExtensions"]
            // }

            // datasource db {
            //   provider   = "postgresql"
            //   url        = env("DATABASE_URL")
            //   extensions = [citext]
            // }

            model Model {
              #id(id, Int, @id)
              slug String @unique @default("") @test.Citext
            }
        "#
        }
        .to_owned()
    }

    #[connector_test]
    async fn write_and_read_back(runner: Runner) -> TestResult<()> {

        Ok(())
    }
}
