use std::fs::{self, File};
use std::path::PathBuf;

use sqlx::{Row, SqlitePool, migrate::Migrator, sqlite::SqliteConnectOptions};
use tokio::runtime::Runtime;
use zdnp_core::{
    Address, AddressDto, AddressRepository, AddressRepositoryError, Migrations, MigrationsResult,
    OrganizationDto, OrganizationRepository, OrganizationRepositoryError,
};

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

pub struct SqliteAddressRepository {
    database_file_name: String,
}

impl SqliteAddressRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_file_name<S: Into<String>>(file_name: S) -> Self {
        Self {
            database_file_name: file_name.into(),
        }
    }

    fn database_path(&self) -> Result<PathBuf, AddressRepositoryError> {
        let executable = std::env::current_exe()
            .map_err(|error| AddressRepositoryError::storage(error.to_string()))?;
        let directory = executable.parent().ok_or_else(|| {
            AddressRepositoryError::storage("Failed to determine application directory")
        })?;

        Ok(directory.join(&self.database_file_name))
    }
}

impl Default for SqliteAddressRepository {
    fn default() -> Self {
        Self {
            database_file_name: DEFAULT_DATABASE_FILE_NAME.to_string(),
        }
    }
}

impl AddressRepository for SqliteAddressRepository {
    fn create(&self, dto: &AddressDto) -> Result<i64, AddressRepositoryError> {
        let database_path = self.database_path()?;
        let runtime =
            Runtime::new().map_err(|error| AddressRepositoryError::storage(error.to_string()))?;

        runtime.block_on(async move {
            let options = SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true);

            let pool = SqlitePool::connect_with(options)
                .await
                .map_err(|error| AddressRepositoryError::storage(error.to_string()))?;

            let region_code = dto
                .region_code
                .as_deref()
                .ok_or_else(|| AddressRepositoryError::storage("Region code is required"))?;

            let result = sqlx::query(
                r#"INSERT INTO address (
                    region_code, note, country, district, city, settlement, street, building, room
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"#,
            )
            .bind(region_code)
            .bind(dto.note.as_deref())
            .bind(dto.country.as_deref())
            .bind(dto.district.as_deref())
            .bind(dto.city.as_deref())
            .bind(dto.settlement.as_deref())
            .bind(dto.street.as_deref())
            .bind(dto.building.as_deref())
            .bind(dto.room.as_deref())
            .execute(&pool)
            .await
            .map_err(|error| AddressRepositoryError::storage(error.to_string()))?;

            let id = result.last_insert_rowid();

            pool.close().await;

            Ok::<i64, AddressRepositoryError>(id)
        })
    }

    fn list(&self) -> Result<Vec<Address>, AddressRepositoryError> {
        let database_path = self.database_path()?;
        let runtime =
            Runtime::new().map_err(|error| AddressRepositoryError::storage(error.to_string()))?;

        runtime.block_on(async move {
            let options = SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true);

            let pool = SqlitePool::connect_with(options)
                .await
                .map_err(|error| AddressRepositoryError::storage(error.to_string()))?;

            let rows = sqlx::query(
                r#"SELECT id, region_code, note, country, district, city, settlement, street, building, room
                   FROM address
                   ORDER BY id"#,
            )
            .fetch_all(&pool)
            .await
            .map_err(|error| AddressRepositoryError::storage(error.to_string()))?;

            pool.close().await;

            let addresses = rows
                .into_iter()
                .map(|row| Address {
                    id: row.get("id"),
                    region_code: row.get("region_code"),
                    note: row.get("note"),
                    country: row.get("country"),
                    district: row.get("district"),
                    city: row.get("city"),
                    settlement: row.get("settlement"),
                    street: row.get("street"),
                    building: row.get("building"),
                    room: row.get("room"),
                })
                .collect();

            Ok::<Vec<Address>, AddressRepositoryError>(addresses)
        })
    }
}

// ---------------- Organization Data Repository ----------------
pub struct SqliteOrganizationRepository {
    database_file_name: String,
}

impl SqliteOrganizationRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_file_name<S: Into<String>>(file_name: S) -> Self {
        Self {
            database_file_name: file_name.into(),
        }
    }

    fn database_path(&self) -> Result<PathBuf, OrganizationRepositoryError> {
        let executable = std::env::current_exe()
            .map_err(|error| OrganizationRepositoryError::storage(error.to_string()))?;
        let directory = executable.parent().ok_or_else(|| {
            OrganizationRepositoryError::storage("Failed to determine application directory")
        })?;

        Ok(directory.join(&self.database_file_name))
    }
}

impl Default for SqliteOrganizationRepository {
    fn default() -> Self {
        Self {
            database_file_name: DEFAULT_DATABASE_FILE_NAME.to_string(),
        }
    }
}

impl OrganizationRepository for SqliteOrganizationRepository {
    fn create(&self, dto: &OrganizationDto) -> Result<i64, OrganizationRepositoryError> {
        let database_path = self.database_path()?;
        let runtime = Runtime::new()
            .map_err(|error| OrganizationRepositoryError::storage(error.to_string()))?;

        runtime.block_on(async move {
            let options = SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true);

            let pool = SqlitePool::connect_with(options)
                .await
                .map_err(|error| OrganizationRepositoryError::storage(error.to_string()))?;

            // Required string fields
            let full_name = dto
                .full_name
                .as_deref()
                .ok_or_else(|| OrganizationRepositoryError::storage("Full name is required"))?;

            let abbreviated_name = dto
                .abbreviated_name
                .as_deref()
                .ok_or_else(|| OrganizationRepositoryError::storage("Abbreviated name is required"))?;

            let inn = dto
                .inn
                .as_deref()
                .ok_or_else(|| OrganizationRepositoryError::storage("INN is required"))?;

            let kpp = dto
                .kpp
                .as_deref()
                .ok_or_else(|| OrganizationRepositoryError::storage("KPP is required"))?;

            let email = dto
                .email
                .as_deref()
                .ok_or_else(|| OrganizationRepositoryError::storage("Email is required"))?;

            // Parse numeric fields for INTEGER columns; on parse failure for optional fields, store NULL
            let ogrn_int: Option<i64> = dto
                .ogrn
                .as_deref()
                .and_then(|v| v.trim().parse::<i64>().ok());

            let rafp_int: Option<i64> = dto
                .rafp
                .as_deref()
                .and_then(|v| v.trim().parse::<i64>().ok());

            let inn_int: i64 = inn.trim().parse::<i64>().map_err(|e| {
                OrganizationRepositoryError::storage(format!("Invalid INN: {e}"))
            })?;
            let kpp_int: i64 = kpp.trim().parse::<i64>().map_err(|e| {
                OrganizationRepositoryError::storage(format!("Invalid KPP: {e}"))
            })?;

            let result = sqlx::query(
                r#"INSERT INTO organization (
                    full_name, abbreviated_name, ogrn, rafp, inn, kpp, address_id, email
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"#,
            )
            .bind(full_name)
            .bind(abbreviated_name)
            .bind(ogrn_int)
            .bind(rafp_int)
            .bind(inn_int)
            .bind(kpp_int)
            .bind(dto.address_id)
            .bind(email)
            .execute(&pool)
            .await
            .map_err(|error| OrganizationRepositoryError::storage(error.to_string()))?;

            let id = result.last_insert_rowid();

            pool.close().await;

            Ok::<i64, OrganizationRepositoryError>(id)
        })
    }
}