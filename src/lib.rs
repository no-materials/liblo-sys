mod lo_sys;

pub use lo_sys::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_basic_functionality() {
        unsafe {
            // Test creating an address
            let url = CString::new("osc.udp://localhost:7770/").unwrap();
            let addr = lo_address_new_from_url(url.as_ptr());
            assert!(!addr.is_null());
            lo_address_free(addr);
        }
    }
}
