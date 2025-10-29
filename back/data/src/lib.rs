use std::fs::{self, File};
use std::path::PathBuf;

use sqlx::{Row, SqlitePool, migrate::Migrator, sqlite::SqliteConnectOptions};
use tokio::runtime::Runtime;
use zdnp_core::{
    Address, AddressDto, AddressRepository, AddressRepositoryError, Entrepreneur, EntrepreneurDto,
    EntrepreneurRepository, EntrepreneurRepositoryError, Migrations, MigrationsResult,
    Organization, OrganizationDto, OrganizationRepository, OrganizationRepositoryError, Person,
    PersonDto, PersonRepository, PersonRepositoryError,
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

            let abbreviated_name = dto.abbreviated_name.as_deref().ok_or_else(|| {
                OrganizationRepositoryError::storage("Abbreviated name is required")
            })?;

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

            let inn_int: i64 = inn
                .trim()
                .parse::<i64>()
                .map_err(|e| OrganizationRepositoryError::storage(format!("Invalid INN: {e}")))?;
            let kpp_int: i64 = kpp
                .trim()
                .parse::<i64>()
                .map_err(|e| OrganizationRepositoryError::storage(format!("Invalid KPP: {e}")))?;

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

    fn list(&self) -> Result<Vec<Organization>, OrganizationRepositoryError> {
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

            let rows = sqlx::query(
                r#"SELECT id, full_name, abbreviated_name, ogrn, rafp, inn, kpp, address_id, email
                   FROM organization
                   ORDER BY id"#,
            )
            .fetch_all(&pool)
            .await
            .map_err(|error| OrganizationRepositoryError::storage(error.to_string()))?;

            pool.close().await;

            let organizations = rows
                .into_iter()
                .map(|row| {
                    let ogrn: Option<i64> = row.get("ogrn");
                    let rafp: Option<i64> = row.get("rafp");
                    let inn: i64 = row.get("inn");
                    let kpp: i64 = row.get("kpp");

                    Organization {
                        id: row.get("id"),
                        full_name: row.get("full_name"),
                        abbreviated_name: row.get("abbreviated_name"),
                        ogrn: ogrn.map(|value| value.to_string()),
                        rafp: rafp.map(|value| value.to_string()),
                        inn: inn.to_string(),
                        kpp: kpp.to_string(),
                        address_id: row.get("address_id"),
                        email: row.get("email"),
                    }
                })
                .collect();

            Ok::<Vec<Organization>, OrganizationRepositoryError>(organizations)
        })
    }
}

// ---------------- Entrepreneur Data Repository ----------------
pub struct SqliteEntrepreneurRepository {
    database_file_name: String,
}

impl SqliteEntrepreneurRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_file_name<S: Into<String>>(file_name: S) -> Self {
        Self {
            database_file_name: file_name.into(),
        }
    }

    fn database_path(&self) -> Result<PathBuf, EntrepreneurRepositoryError> {
        let executable = std::env::current_exe()
            .map_err(|error| EntrepreneurRepositoryError::storage(error.to_string()))?;
        let directory = executable.parent().ok_or_else(|| {
            EntrepreneurRepositoryError::storage("Failed to determine application directory")
        })?;

        Ok(directory.join(&self.database_file_name))
    }
}

impl Default for SqliteEntrepreneurRepository {
    fn default() -> Self {
        Self {
            database_file_name: DEFAULT_DATABASE_FILE_NAME.to_string(),
        }
    }
}

impl EntrepreneurRepository for SqliteEntrepreneurRepository {
    fn create(&self, dto: &EntrepreneurDto) -> Result<i64, EntrepreneurRepositoryError> {
        let database_path = self.database_path()?;
        let runtime = Runtime::new()
            .map_err(|error| EntrepreneurRepositoryError::storage(error.to_string()))?;

        runtime.block_on(async move {
            let options = SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true);

            let pool = SqlitePool::connect_with(options)
                .await
                .map_err(|error| EntrepreneurRepositoryError::storage(error.to_string()))?;

            let surname = dto
                .surname
                .as_deref()
                .ok_or_else(|| EntrepreneurRepositoryError::storage("Surname is required"))?;
            let name = dto
                .name
                .as_deref()
                .ok_or_else(|| EntrepreneurRepositoryError::storage("Name is required"))?;
            let ogrnip = dto
                .ogrnip
                .as_deref()
                .ok_or_else(|| EntrepreneurRepositoryError::storage("OGRNIP is required"))?;
            let inn = dto
                .inn
                .as_deref()
                .ok_or_else(|| EntrepreneurRepositoryError::storage("INN is required"))?;

            let ogrnip_int: i64 = ogrnip.trim().parse().map_err(|error| {
                EntrepreneurRepositoryError::storage(format!("Invalid OGRNIP: {error}"))
            })?;
            let inn_int: i64 = inn.trim().parse().map_err(|error| {
                EntrepreneurRepositoryError::storage(format!("Invalid INN: {error}"))
            })?;

            let result = sqlx::query(
                r#"INSERT INTO entrepreneur (
                    surname, name, patronymic, ogrnip, inn, address_id, email
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"#,
            )
            .bind(surname)
            .bind(name)
            .bind(dto.patronymic.as_deref())
            .bind(ogrnip_int)
            .bind(inn_int)
            .bind(dto.address_id)
            .bind(dto.email.as_deref())
            .execute(&pool)
            .await
            .map_err(|error| EntrepreneurRepositoryError::storage(error.to_string()))?;

            let id = result.last_insert_rowid();

            pool.close().await;

            Ok::<i64, EntrepreneurRepositoryError>(id)
        })
    }

    fn list(&self) -> Result<Vec<Entrepreneur>, EntrepreneurRepositoryError> {
        let database_path = self.database_path()?;
        let runtime = Runtime::new()
            .map_err(|error| EntrepreneurRepositoryError::storage(error.to_string()))?;

        runtime.block_on(async move {
            let options = SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true);

            let pool = SqlitePool::connect_with(options)
                .await
                .map_err(|error| EntrepreneurRepositoryError::storage(error.to_string()))?;

            let rows = sqlx::query(
                r#"SELECT id, surname, name, patronymic, ogrnip, inn, address_id, email
                   FROM entrepreneur
                   ORDER BY id"#,
            )
            .fetch_all(&pool)
            .await
            .map_err(|error| EntrepreneurRepositoryError::storage(error.to_string()))?;

            pool.close().await;

            let entrepreneurs = rows
                .into_iter()
                .map(|row| {
                    let ogrnip: i64 = row.get("ogrnip");
                    let inn: i64 = row.get("inn");

                    Entrepreneur {
                        id: row.get("id"),
                        surname: row.get("surname"),
                        name: row.get("name"),
                        patronymic: row.get("patronymic"),
                        ogrnip: ogrnip.to_string(),
                        inn: inn.to_string(),
                        address_id: row.get("address_id"),
                        email: row.get("email"),
                    }
                })
                .collect();

            Ok::<Vec<Entrepreneur>, EntrepreneurRepositoryError>(entrepreneurs)
        })
    }
}

// ---------------- Person Data Repository ----------------

pub struct SqlitePersonRepository {
    database_file_name: String,
}

impl SqlitePersonRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_file_name<S: Into<String>>(file_name: S) -> Self {
        Self {
            database_file_name: file_name.into(),
        }
    }

    fn database_path(&self) -> Result<PathBuf, PersonRepositoryError> {
        let executable = std::env::current_exe()
            .map_err(|error| PersonRepositoryError::storage(error.to_string()))?;
        let directory = executable.parent().ok_or_else(|| {
            PersonRepositoryError::storage("Failed to determine application directory")
        })?;

        Ok(directory.join(&self.database_file_name))
    }
}

impl Default for SqlitePersonRepository {
    fn default() -> Self {
        Self {
            database_file_name: DEFAULT_DATABASE_FILE_NAME.to_string(),
        }
    }
}

impl PersonRepository for SqlitePersonRepository {
    fn create(&self, dto: &PersonDto) -> Result<i64, PersonRepositoryError> {
        let database_path = self.database_path()?;
        let runtime =
            Runtime::new().map_err(|error| PersonRepositoryError::storage(error.to_string()))?;

        runtime.block_on(async move {
            let options = SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true);

            let pool = SqlitePool::connect_with(options)
                .await
                .map_err(|error| PersonRepositoryError::storage(error.to_string()))?;

            let name = dto
                .name
                .as_deref()
                .ok_or_else(|| PersonRepositoryError::storage("Name is required"))?;
            let surname = dto
                .surname
                .as_deref()
                .ok_or_else(|| PersonRepositoryError::storage("Surname is required"))?;
            let snils = dto
                .snils
                .as_deref()
                .ok_or_else(|| PersonRepositoryError::storage("SNILS is required"))?;
            let email = dto
                .email
                .as_deref()
                .ok_or_else(|| PersonRepositoryError::storage("Email is required"))?;

            let snils_int: i64 = snils.trim().parse().map_err(|error| {
                PersonRepositoryError::storage(format!("Invalid SNILS: {error}"))
            })?;

            let result = sqlx::query(
                r#"INSERT INTO person (
                    name, patronymic, surname, snils, email, address_id
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
            )
            .bind(name)
            .bind(dto.patronymic.as_deref())
            .bind(surname)
            .bind(snils_int)
            .bind(email)
            .bind(dto.address_id)
            .execute(&pool)
            .await
            .map_err(|error| PersonRepositoryError::storage(error.to_string()))?;

            let id = result.last_insert_rowid();

            pool.close().await;

            Ok::<i64, PersonRepositoryError>(id)
        })
    }

    fn list(&self) -> Result<Vec<Person>, PersonRepositoryError> {
        let database_path = self.database_path()?;
        let runtime =
            Runtime::new().map_err(|error| PersonRepositoryError::storage(error.to_string()))?;

        runtime.block_on(async move {
            let options = SqliteConnectOptions::new()
                .filename(&database_path)
                .create_if_missing(true);

            let pool = SqlitePool::connect_with(options)
                .await
                .map_err(|error| PersonRepositoryError::storage(error.to_string()))?;

            let rows = sqlx::query(
                r#"SELECT id, name, patronymic, surname, snils, email, address_id
                   FROM person
                   ORDER BY id"#,
            )
            .fetch_all(&pool)
            .await
            .map_err(|error| PersonRepositoryError::storage(error.to_string()))?;

            pool.close().await;

            let persons = rows
                .into_iter()
                .map(|row| {
                    let snils: i64 = row.get("snils");

                    Person {
                        id: row.get("id"),
                        name: row.get("name"),
                        patronymic: row.get("patronymic"),
                        surname: row.get("surname"),
                        snils: snils.to_string(),
                        email: row.get("email"),
                        address_id: row.get("address_id"),
                    }
                })
                .collect();

            Ok::<Vec<Person>, PersonRepositoryError>(persons)
        })
    }
}
