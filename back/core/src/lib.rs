pub type MigrationsResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait Migrations {
    fn run(&self) -> MigrationsResult<()>;
}

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct AddressDto {
    pub region_code: Option<String>,
    pub note: Option<String>,
    pub country: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub settlement: Option<String>,
    pub street: Option<String>,
    pub building: Option<String>,
    pub room: Option<String>,
}

pub fn format_address(dto: &AddressDto) -> String {
    format_address_impl(dto)
}

fn format_address_impl(dto: &AddressDto) -> String {
    fn push_if_present(parts: &mut Vec<String>, value: &Option<String>) {
        if let Some(value) = value {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                parts.push(trimmed.to_string());
            }
        }
    }

    let mut parts = Vec::new();
    push_if_present(&mut parts, &dto.region_code);
    push_if_present(&mut parts, &dto.note);
    push_if_present(&mut parts, &dto.country);
    push_if_present(&mut parts, &dto.district);
    push_if_present(&mut parts, &dto.city);
    push_if_present(&mut parts, &dto.settlement);
    push_if_present(&mut parts, &dto.street);
    push_if_present(&mut parts, &dto.building);
    push_if_present(&mut parts, &dto.room);

    parts.join(", ")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Address {
    pub id: i64,
    pub region_code: String,
    pub note: Option<String>,
    pub country: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub settlement: Option<String>,
    pub street: Option<String>,
    pub building: Option<String>,
    pub room: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressRepositoryError {
    Storage(String),
}

impl AddressRepositoryError {
    pub fn storage<S: Into<String>>(message: S) -> Self {
        Self::Storage(message.into())
    }
}

impl std::fmt::Display for AddressRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for AddressRepositoryError {}

pub trait AddressRepository {
    fn create(&self, dto: &AddressDto) -> Result<i64, AddressRepositoryError>;
    fn list(&self) -> Result<Vec<Address>, AddressRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressError {
    MissingRegionCode,
    Repository(AddressRepositoryError),
}

impl std::fmt::Display for AddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRegionCode => f.write_str("Region code is required"),
            Self::Repository(error) => write!(f, "Repository error: {error}"),
        }
    }
}

impl std::error::Error for AddressError {}

pub fn create_address<R: AddressRepository>(
    repository: &R,
    dto: &AddressDto,
) -> Result<i64, AddressError> {
    let sanitized = sanitize_address(dto)?;

    repository
        .create(&sanitized)
        .map_err(AddressError::Repository)
}

pub fn list_addresses<R: AddressRepository>(
    repository: &R,
) -> Result<Vec<Address>, AddressRepositoryError> {
    repository.list()
}

fn sanitize_address(dto: &AddressDto) -> Result<AddressDto, AddressError> {
    fn sanitize_field(value: &Option<String>) -> Option<String> {
        value
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string())
    }

    let region_code = dto
        .region_code
        .as_ref()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
        .ok_or(AddressError::MissingRegionCode)?;

    Ok(AddressDto {
        region_code: Some(region_code),
        note: sanitize_field(&dto.note),
        country: sanitize_field(&dto.country),
        district: sanitize_field(&dto.district),
        city: sanitize_field(&dto.city),
        settlement: sanitize_field(&dto.settlement),
        street: sanitize_field(&dto.street),
        building: sanitize_field(&dto.building),
        room: sanitize_field(&dto.room),
    })
}

// ---------------- Organization Core API ----------------
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OrganizationDto {
    pub full_name: Option<String>,
    pub abbreviated_name: Option<String>,
    pub ogrn: Option<String>,
    pub rafp: Option<String>,
    pub inn: Option<String>,
    pub kpp: Option<String>,
    pub address_id: i64,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationRepositoryError {
    Storage(String),
}

impl OrganizationRepositoryError {
    pub fn storage<S: Into<String>>(message: S) -> Self {
        Self::Storage(message.into())
    }
}

impl std::fmt::Display for OrganizationRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for OrganizationRepositoryError {}

pub trait OrganizationRepository {
    fn create(&self, dto: &OrganizationDto) -> Result<i64, OrganizationRepositoryError>;
    fn list(&self) -> Result<Vec<Organization>, OrganizationRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrganizationError {
    MissingFullName,
    MissingAbbreviatedName,
    MissingInn,
    MissingKpp,
    MissingEmail,
    Repository(OrganizationRepositoryError),
}

impl std::fmt::Display for OrganizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingFullName => f.write_str("Full name is required"),
            Self::MissingAbbreviatedName => f.write_str("Abbreviated name is required"),
            Self::MissingInn => f.write_str("INN is required"),
            Self::MissingKpp => f.write_str("KPP is required"),
            Self::MissingEmail => f.write_str("Email is required"),
            Self::Repository(error) => write!(f, "Repository error: {error}"),
        }
    }
}

impl std::error::Error for OrganizationError {}

pub fn create_organization<R: OrganizationRepository>(
    repository: &R,
    dto: &OrganizationDto,
) -> Result<i64, OrganizationError> {
    let sanitized = sanitize_organization(dto)?;
    repository
        .create(&sanitized)
        .map_err(OrganizationError::Repository)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Organization {
    pub id: i64,
    pub full_name: String,
    pub abbreviated_name: String,
    pub ogrn: Option<String>,
    pub rafp: Option<String>,
    pub inn: String,
    pub kpp: String,
    pub address_id: i64,
    pub email: String,
}

pub fn list_organizations<R: OrganizationRepository>(
    repository: &R,
) -> Result<Vec<Organization>, OrganizationRepositoryError> {
    repository.list()
}

fn sanitize_organization(dto: &OrganizationDto) -> Result<OrganizationDto, OrganizationError> {
    fn sanitize_field(value: &Option<String>) -> Option<String> {
        value
            .as_ref()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
    }

    let full_name = dto
        .full_name
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(OrganizationError::MissingFullName)?;

    let abbreviated_name = dto
        .abbreviated_name
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(OrganizationError::MissingAbbreviatedName)?;

    let inn = dto
        .inn
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(OrganizationError::MissingInn)?;

    let kpp = dto
        .kpp
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(OrganizationError::MissingKpp)?;

    let email = dto
        .email
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(OrganizationError::MissingEmail)?;

    Ok(OrganizationDto {
        full_name: Some(full_name),
        abbreviated_name: Some(abbreviated_name),
        ogrn: sanitize_field(&dto.ogrn),
        rafp: sanitize_field(&dto.rafp),
        inn: Some(inn),
        kpp: Some(kpp),
        address_id: dto.address_id,
        email: Some(email),
    })
}

// ---------------- Entrepreneur Core API ----------------
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntrepreneurDto {
    pub surname: Option<String>,
    pub name: Option<String>,
    pub patronymic: Option<String>,
    pub ogrnip: Option<String>,
    pub inn: Option<String>,
    pub address_id: i64,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntrepreneurRepositoryError {
    Storage(String),
}

impl EntrepreneurRepositoryError {
    pub fn storage<S: Into<String>>(message: S) -> Self {
        Self::Storage(message.into())
    }
}

impl std::fmt::Display for EntrepreneurRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for EntrepreneurRepositoryError {}

pub trait EntrepreneurRepository {
    fn create(&self, dto: &EntrepreneurDto) -> Result<i64, EntrepreneurRepositoryError>;
    fn list(&self) -> Result<Vec<Entrepreneur>, EntrepreneurRepositoryError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntrepreneurError {
    MissingSurname,
    MissingName,
    MissingOgrnip,
    MissingInn,
    Repository(EntrepreneurRepositoryError),
}

impl std::fmt::Display for EntrepreneurError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSurname => f.write_str("Surname is required"),
            Self::MissingName => f.write_str("Name is required"),
            Self::MissingOgrnip => f.write_str("OGRNIP is required"),
            Self::MissingInn => f.write_str("INN is required"),
            Self::Repository(error) => write!(f, "Repository error: {error}"),
        }
    }
}

impl std::error::Error for EntrepreneurError {}

pub fn create_entrepreneur<R: EntrepreneurRepository>(
    repository: &R,
    dto: &EntrepreneurDto,
) -> Result<i64, EntrepreneurError> {
    let sanitized = sanitize_entrepreneur(dto)?;
    repository
        .create(&sanitized)
        .map_err(EntrepreneurError::Repository)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Entrepreneur {
    pub id: i64,
    pub surname: String,
    pub name: String,
    pub patronymic: Option<String>,
    pub ogrnip: String,
    pub inn: String,
    pub address_id: i64,
    pub email: Option<String>,
}

pub fn list_entrepreneurs<R: EntrepreneurRepository>(
    repository: &R,
) -> Result<Vec<Entrepreneur>, EntrepreneurRepositoryError> {
    repository.list()
}

fn sanitize_entrepreneur(dto: &EntrepreneurDto) -> Result<EntrepreneurDto, EntrepreneurError> {
    fn sanitize_field(value: &Option<String>) -> Option<String> {
        value
            .as_ref()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
    }

    let surname = dto
        .surname
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(EntrepreneurError::MissingSurname)?;

    let name = dto
        .name
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(EntrepreneurError::MissingName)?;

    let ogrnip = dto
        .ogrnip
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(EntrepreneurError::MissingOgrnip)?;

    let inn = dto
        .inn
        .as_ref()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .ok_or(EntrepreneurError::MissingInn)?;

    Ok(EntrepreneurDto {
        surname: Some(surname),
        name: Some(name),
        patronymic: sanitize_field(&dto.patronymic),
        ogrnip: Some(ogrnip),
        inn: Some(inn),
        address_id: dto.address_id,
        email: sanitize_field(&dto.email),
    })
}

// Internal Rust API (can be used from Rust unit tests or other Rust crates if needed)
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_i32(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_i32_handles_negatives() {
        assert_eq!(add_i32(-5, 2), -3);
    }

    #[test]
    fn format_address_skips_empty_values() {
        let dto = AddressDto {
            region_code: Some("77".into()),
            note: Some("Около метро".into()),
            country: Some("Россия".into()),
            district: Some("ЦАО".into()),
            city: Some("Москва".into()),
            settlement: Some("".into()),
            street: Some("Тверская".into()),
            building: Some("1".into()),
            room: Some("101".into()),
        };

        let formatted = format_address(&dto);

        assert_eq!(
            formatted,
            "77, Около метро, Россия, ЦАО, Москва, Тверская, 1, 101"
        );
    }

    #[test]
    fn create_address_requires_region_code() {
        let repository = RecordingRepository::default();
        let dto = AddressDto::default();

        let result = create_address(&repository, &dto);

        match result {
            Err(AddressError::MissingRegionCode) => {}
            _ => panic!("Expected MissingRegionCode error"),
        }
    }

    #[test]
    fn create_address_sanitizes_fields() {
        let repository = RecordingRepository::default();
        let dto = AddressDto {
            region_code: Some(" 77 ".into()),
            city: Some(" Москва ".into()),
            street: Some(" ".into()),
            ..Default::default()
        };

        let id = create_address(&repository, &dto).expect("address should be created");
        assert_eq!(id, 42);

        let captured = repository.last();
        let captured = captured.as_ref().expect("repository should capture dto");

        assert_eq!(captured.region_code.as_deref(), Some("77"));
        assert_eq!(captured.city.as_deref(), Some("Москва"));
        assert_eq!(captured.street, None);
    }

    #[derive(Default)]
    struct RecordingRepository {
        last: RefCell<Option<AddressDto>>,
    }

    impl RecordingRepository {
        fn last(&self) -> Option<AddressDto> {
            self.last.borrow().clone()
        }
    }

    impl AddressRepository for RecordingRepository {
        fn create(&self, dto: &AddressDto) -> Result<i64, AddressRepositoryError> {
            *self.last.borrow_mut() = Some(dto.clone());
            Ok(42)
        }

        fn list(&self) -> Result<Vec<Address>, AddressRepositoryError> {
            Ok(Vec::new())
        }
    }
}
