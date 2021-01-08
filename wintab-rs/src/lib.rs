use std::{ffi::c_void, os::raw::c_uint};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// UINT API WTInfoA(UINT, UINT, LPVOID);
extern "C" {
    pub fn WTInfoA(wCategory: c_uint,
        nIndex: c_uint,
        lpOutput: *mut c_void) -> c_uint;
}
