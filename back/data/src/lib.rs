use std::fs::{self, File};
use std::path::PathBuf;

use sqlx::{SqlitePool, migrate::Migrator, sqlite::SqliteConnectOptions};
use tokio::runtime::Runtime;
use zdnp_core::{Migrations, MigrationsResult};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

const DEFAULT_DATABASE_FILE_NAME: &str = "zdnp.sqlite";

pub struct SqliteMigrations {
    database_file_name: String,
}

impl Default for SqliteMigrations {
    fn default() -> Self {
        Self {
            database_file_name: DEFAULT_DATABASE_FILE_NAME.to_string(),
        }
    }
}

impl SqliteMigrations {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_file_name<S: Into<String>>(file_name: S) -> Self {
        Self {
            database_file_name: file_name.into(),
        }
    }

    fn database_path(&self) -> MigrationsResult<PathBuf> {
        let executable = std::env::current_exe()?;
        let directory = executable.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to determine application directory",
            )
        })?;

        Ok(directory.join(&self.database_file_name))
    }
}

impl Migrations for SqliteMigrations {
    fn run(&self) -> MigrationsResult<()> {
        let database_path = self.database_path()?;

        if let Some(parent) = database_path.parent() {
            fs::create_dir_all(parent)?;
        }

        if !database_path.exists() {
            File::create(&database_path)?;
        }

        let runtime = Runtime::new()?;

        runtime.block_on(async {
            let options = SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true);

            let pool = SqlitePool::connect_with(options).await?;
            MIGRATOR.run(&pool).await?;
            pool.close().await;
            Ok::<(), sqlx::Error>(())
        })?;

        Ok(())
    }
}
