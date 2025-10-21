use std::ffi::{CStr, CString, c_char};
use std::str::Utf8Error;

use zdnp_core::{self, AddressDto, Migrations};

/// Errors that can occur while converting FFI data into safe Rust structures.
#[derive(Debug)]
pub enum FfiConversionError {
    InvalidUtf8,
}

impl From<Utf8Error> for FfiConversionError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidUtf8
    }
}

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
    /// Converts this C-friendly representation into a safe [`AddressDto`].
    ///
    /// # Safety
    /// All pointers must either be null or reference valid null-terminated UTF-8 strings.
    unsafe fn try_into_core(&self) -> Result<AddressDto, FfiConversionError> {
        fn read_field(ptr: *const c_char) -> Result<Option<String>, FfiConversionError> {
            if ptr.is_null() {
                return Ok(None);
            }

            // SAFETY: The caller guarantees that the pointer is valid for reads and points to a
            // null-terminated string.
            let c_str = unsafe { CStr::from_ptr(ptr) };
            let utf8 = c_str.to_str()?;
            if utf8.is_empty() {
                Ok(None)
            } else {
                Ok(Some(utf8.to_owned()))
            }
        }

        Ok(AddressDto {
            postal_code: read_field(self.postal_code)?,
            country: read_field(self.country)?,
            region: read_field(self.region)?,
            district: read_field(self.district)?,
            city: read_field(self.city)?,
            locality: read_field(self.locality)?,
            street: read_field(self.street)?,
            house: read_field(self.house)?,
            building: read_field(self.building)?,
            structure: read_field(self.structure)?,
            apartment: read_field(self.apartment)?,
            comment: read_field(self.comment)?,
        })
    }
}

/// Formats the incoming address DTO and returns a newly allocated C string.
///
/// # Safety
/// The caller must pass a valid pointer to [`AddressDtoFfi`]. Each field should be a
/// null-terminated UTF-8 encoded string or null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn core_format_address(dto: *const AddressDtoFfi) -> *mut c_char {
    let dto = match unsafe { dto.as_ref() } {
        Some(dto) => dto,
        None => return std::ptr::null_mut(),
    };

    let dto = match unsafe { dto.try_into_core() } {
        Ok(dto) => dto,
        Err(_) => return std::ptr::null_mut(),
    };

    let formatted = zdnp_core::format_address(&dto);

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
    zdnp_core::add_i32(a, b)
}

/// Returns a pointer to a null-terminated static string with version info.
/// Do NOT attempt to free this pointer on the C# side.
#[unsafe(no_mangle)]
pub extern "C" fn core_version() -> *const c_char {
    b"0.1.0\0".as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn migrations_run() -> bool {
    let migrations = zdnp_data::SqliteMigrations::new();
    migrations.run().is_ok()
}
