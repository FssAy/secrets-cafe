mod surql;
mod types;
mod calls;

use std::ops::Deref;
use once_cell::sync::OnceCell;
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;

type SurrealClient = Surreal<Db>;

const DB_PATH: &str = "secrets-cafe.db";

static DB_INST: OnceCell<Database> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Database {
    client: SurrealClient,
}

impl Database {
    /// Initializes the database.
    ///
    /// Run only once!
    async fn init() -> anyhow::Result<Self> {
        let db_config = surrealdb::opt::Config::default()
            .strict();

        let client: SurrealClient = Surreal::new::<RocksDb>(
            (DB_PATH, db_config)
        ).await?;

        let db = Self { client };
        db.build().await?;

        // do not change these EVER!
        db.client.use_ns("cafe").use_db("main").await?;

        Ok(db)
    }

    /// Get the database instance or initialize it if not present.
    ///
    /// Can return an error only when creating a new database instance,
    /// so it's safe to unwrap result after calling it once with a success.
    pub async fn get() -> anyhow::Result<Database> {
        if let Some(db) = DB_INST.get() {
            Ok(db.clone())
        } else {
            let db = Self::init().await?;

            // this function should never fail due to the if statement above
            DB_INST.set(db.clone())
                .expect("critical error, database instance initialized before initialization");

            Ok(db)
        }
    }

    /// Builds the database by calling the `build` query.
    async fn build(&self) -> anyhow::Result<()> {
        self.query(surql::BUILD).await?.check()?;
        Ok(())
    }
}

impl Deref for Database {
    type Target = SurrealClient;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}
