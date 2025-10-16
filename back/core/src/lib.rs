#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AddressDto {
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub locality: Option<String>,
    pub street: Option<String>,
    pub house: Option<String>,
    pub building: Option<String>,
    pub structure: Option<String>,
    pub apartment: Option<String>,
    pub comment: Option<String>,
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
    push_if_present(&mut parts, &dto.postal_code);
    push_if_present(&mut parts, &dto.country);
    push_if_present(&mut parts, &dto.region);
    push_if_present(&mut parts, &dto.district);
    push_if_present(&mut parts, &dto.city);
    push_if_present(&mut parts, &dto.locality);
    push_if_present(&mut parts, &dto.street);
    push_if_present(&mut parts, &dto.house);
    push_if_present(&mut parts, &dto.building);
    push_if_present(&mut parts, &dto.structure);
    push_if_present(&mut parts, &dto.apartment);
    push_if_present(&mut parts, &dto.comment);

    parts.join(", ")
}

pub mod ffi {
    use super::{AddressDto, format_address_impl};
    use std::ffi::{CStr, CString, c_char};

    #[repr(C)]
    pub struct AddressDtoFfi {
        pub postal_code: *const c_char,
        pub country: *const c_char,
        pub region: *const c_char,
        pub district: *const c_char,
        pub city: *const c_char,
        pub locality: *const c_char,
        pub street: *const c_char,
        pub house: *const c_char,
        pub building: *const c_char,
        pub structure: *const c_char,
        pub apartment: *const c_char,
        pub comment: *const c_char,
    }

    impl AddressDtoFfi {
        unsafe fn as_dto(&self) -> AddressDto {
            fn c_str_to_option(ptr: *const c_char) -> Option<String> {
                if ptr.is_null() {
                    return None;
                }

                // SAFETY: Caller guarantees that the pointer is valid and points to a
                // null-terminated string for the duration of the call.
                unsafe { CStr::from_ptr(ptr) }
                    .to_str()
                    .ok()
                    .map(|s| s.to_string())
                    .filter(|s| !s.is_empty())
            }

            AddressDto {
                postal_code: c_str_to_option(self.postal_code),
                country: c_str_to_option(self.country),
                region: c_str_to_option(self.region),
                district: c_str_to_option(self.district),
                city: c_str_to_option(self.city),
                locality: c_str_to_option(self.locality),
                street: c_str_to_option(self.street),
                house: c_str_to_option(self.house),
                building: c_str_to_option(self.building),
                structure: c_str_to_option(self.structure),
                apartment: c_str_to_option(self.apartment),
                comment: c_str_to_option(self.comment),
            }
        }
    }

    /// Formats the incoming address DTO and returns a newly allocated C string.
    ///
    /// # Safety
    /// The caller must pass a valid pointer to [`AddressDtoFfi`]. Each field should be a
    /// null-terminated UTF-8 encoded string or null.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn core_format_address(dto: *const AddressDtoFfi) -> *mut c_char {
        if dto.is_null() {
            return std::ptr::null_mut();
        }

        let dto = unsafe { &*dto };
        let formatted = format_address_impl(&unsafe { dto.as_dto() });

        match CString::new(formatted) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }

    /// Frees a string returned from [`core_format_address`].
    ///
    /// # Safety
    /// The pointer must originate from [`core_format_address`] and must not be freed twice.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn core_free_string(ptr: *mut c_char) {
        if ptr.is_null() {
            return;
        }

        unsafe {
            let _ = CString::from_raw(ptr);
        };
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn core_add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Returns a pointer to a null-terminated static string with version info.
    /// Do NOT attempt to free this pointer on the C# side.
    #[unsafe(no_mangle)]
    pub extern "C" fn core_version() -> *const c_char {
        b"0.1.0\0".as_ptr() as *const c_char
    }
}

// Internal Rust API (can be used from Rust unit tests or other Rust crates if needed)
pub fn add(left: u64, right: u64) -> u64 {
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
    fn format_address_skips_empty_values() {
        let dto = AddressDto {
            postal_code: Some("123456".into()),
            country: Some("Россия".into()),
            region: Some("Московская область".into()),
            district: None,
            city: Some("Москва".into()),
            locality: Some("".into()),
            street: Some("Тверская".into()),
            house: Some("1".into()),
            building: Some("".into()),
            structure: None,
            apartment: Some("101".into()),
            comment: Some("Около метро".into()),
        };

        let formatted = format_address(&dto);

        assert_eq!(
            formatted,
            "123456, Россия, Московская область, Москва, Тверская, 1, 101, Около метро"
        );
    }
}
