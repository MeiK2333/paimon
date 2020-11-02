#[macro_use]
extern crate ctor;
#[macro_use]
extern crate log;
extern crate android_log;
extern crate libc;

use jni::sys::JNINativeMethod;
use jni::JNIEnv;
use libc::c_void;
use std::ffi::CString;
use std::fs;
use std::os::raw::c_char;

#[allow(dead_code)]
extern "C" {
    fn xhook_register(
        pathname_regex_str: *const c_char,
        symbol: *const c_char,
        new_func: *const c_void,
        old_func: *mut *const c_void,
    ) -> i32;
    // 自己实现 yhook 函数来适配 rust 的类型系统
    fn yhook_register(
        pathname_regex_str: *const c_char,
        symbol: *const c_char,
        new_func: *const c_void,
    ) -> *const *const c_void;
    fn xhook_ignore(pathname_regex_str: *const c_char, symbol: *const c_char) -> i32;
    fn xhook_refresh(r#async: i32) -> i32;
    fn xhook_clear();
    fn xhook_enable_debug(flag: i32);
    fn xhook_enable_sigsegv_protection(flag: i32);
}

static mut OLD_JNI_REGISTER_NATIVE_METHODS: *const extern "C" fn(
    *const JNIEnv,
    *const c_char,
    *const JNINativeMethod,
    i32,
) -> i32 = std::ptr::null();

#[no_mangle]
extern "C" fn new_jniRegisterNativeMethods(
    env: *const JNIEnv,
    class_name: *const c_char,
    methods: *const JNINativeMethod,
    num_methods: i32,
) -> i32 {
    debug!("new jniRegisterNativeMethods");
    unsafe { (*OLD_JNI_REGISTER_NATIVE_METHODS)(env, class_name, methods, num_methods) }
}

#[ctor]
unsafe fn constructor() {
    android_log::init("paimon").unwrap();

    info!("Hello Paimon!");

    if libc::getuid() != 0 {
        warn!("not root!");
        return;
    }

    let cmdline = read_file_to_string("/proc/self/cmdline");
    if cmdline != "zygote"
        && cmdline != "zygote32"
        && cmdline != "zygote64"
        && cmdline != "usap32"
        && cmdline != "usap64"
    {
        warn!("not zygote (cmd = {})", cmdline);
        return;
    }
    info!("zygote");

    let pathname_regex_str = CString::new(".*\\libandroid_runtime.so$").unwrap();
    let symbol = CString::new("jniRegisterNativeMethods").unwrap();
    debug!("{:?}, {:?}", pathname_regex_str, symbol);

    let old_func = yhook_register(
        pathname_regex_str.as_ptr(),
        symbol.as_ptr(),
        new_jniRegisterNativeMethods as *const c_void,
    )
        as *const extern "C" fn(*const JNIEnv, *const c_char, *const JNINativeMethod, i32) -> i32;
    OLD_JNI_REGISTER_NATIVE_METHODS = old_func;

    if xhook_refresh(0) == 0 {
        xhook_clear();
        info!("hook installed");
    } else {
        error!("failed to refresh hook");
    }
}

fn read_file_to_string(file: &str) -> String {
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
