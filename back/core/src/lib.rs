pub type MigrationsResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub trait Migrations {
    fn run(&self) -> MigrationsResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
    }
}
