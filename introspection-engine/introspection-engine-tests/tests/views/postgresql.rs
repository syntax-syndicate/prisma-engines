use indoc::indoc;
use introspection_engine_tests::test_api::*;

#[test_connector(tags(Postgres), exclude(CockroachDb), preview_features("views"))]
async fn simple_view_from_one_table(api: &TestApi) -> TestResult {
    let setup = indoc! {r#"
        CREATE TABLE "User" (
            id SERIAL PRIMARY KEY,
            first_name VARCHAR(255) NOT NULL,
            last_name VARCHAR(255) NULL
        );

        CREATE VIEW "Schwuser" AS
            SELECT id, first_name, last_name FROM "User";
    "#};

    api.raw_cmd(setup).await;

    let expected = expect![[r#"
        generator client {
          provider        = "prisma-client-js"
          previewFeatures = ["views"]
        }

        datasource db {
          provider = "postgresql"
          url      = "env(TEST_DATABASE_URL)"
        }

        model User {
          id         Int     @id @default(autoincrement())
          first_name String  @db.VarChar(255)
          last_name  String? @db.VarChar(255)
        }

        view Schwuser {
          id         Int?
          first_name String? @db.VarChar(255)
          last_name  String? @db.VarChar(255)
        }
    "#]];

    api.expect_datamodel(&expected).await;

    Ok(())
}
