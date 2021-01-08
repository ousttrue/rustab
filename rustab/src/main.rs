use std::{
    ffi::c_void,
    os::raw::{c_char, c_uint},
};

extern crate kernel32;
// extern crate wintab_rs;

extern "C" {

    // fn WTInfoA(wCategory: c_uint, nIndex: c_uint, lpOutput: *mut c_void) -> c_uint;

}

fn main() {
    unsafe {
        let dll_name = b"wintab32.dll\0".as_ptr() as *const c_char;
        let dll = kernel32::LoadLibraryA(dll_name);

        let func_name = b"WTInfoA\0".as_ptr() as *const c_char;
        let p = kernel32::GetProcAddress(dll, func_name);

        let WTInfoA = std::mem::transmute::<*const c_void, fn(c_uint, c_uint, *mut c_void) -> c_uint>(p);

        WTInfoA(0, 0, std::ptr::null_mut());
        println!("Hello, world!");
    }
}
