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
}
