use std::ffi::CStr;
use std::fs;
use std::os::raw::c_char;

pub fn read_file_to_string(file: &str) -> String {
    let data = fs::read_to_string(file).unwrap();
    let mut ret = "".to_owned();
    for ch in data.chars() {
        if ch as i32 != 0 {
            ret.push(ch);
        } else {
            break;
        }
    }
    ret
}

pub fn string_from_c_buf(buf: *const c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(buf) };
    c_str.to_str().unwrap().to_string()
}
