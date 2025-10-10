use std::ffi::c_char;

// Internal Rust API (can be used from Rust unit tests or other Rust crates if needed)
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// C ABI exports for interop with .NET (WPF)
#[unsafe(no_mangle)]
pub extern "C" fn core_add(a: i32, b: i32) -> i32 {
    a + b
}

// Returns a pointer to a null-terminated static string with version info.
// Do NOT attempt to free this pointer on the C# side.
#[unsafe(no_mangle)]
pub extern "C" fn core_version() -> *const c_char {
    b"0.1.0\0".as_ptr() as *const c_char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
