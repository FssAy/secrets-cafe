mod surql;
pub mod types;
mod calls;

use std::ops::Deref;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;

type SurrealClient = Surreal<Db>;

#[cfg(not(test))]
const DB_PATH: &str = "secrets-cafe.db";

#[cfg(not(test))]
static DB_INST: once_cell::sync::OnceCell<Database> = once_cell::sync::OnceCell::new();

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

        #[cfg(not(test))]
        let client: SurrealClient = Surreal::new::<surrealdb::engine::local::RocksDb>(
            (DB_PATH, db_config)
        ).await?;

        #[cfg(test)]
        let client: SurrealClient = Surreal::new::<surrealdb::engine::local::Mem>(
            db_config
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
    #[cfg(not(test))]
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

    #[cfg(test)]
    pub async fn get() -> anyhow::Result<Database> {
        Self::init().await
    }

    /// Builds the database by executing the `build` query.
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
