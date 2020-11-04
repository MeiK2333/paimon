#[macro_use]
extern crate ctor;
#[macro_use]
extern crate log;
extern crate android_log;
extern crate libc;

mod native_methods;
mod utils;

use jni::objects::JClass;
use jni::sys::{jboolean, jint, jintArray, jobjectArray, jstring, JNINativeMethod};
use jni::JNIEnv;
use libc::c_void;
use native_methods::native_fork_and_specialize_hook;
use std::ffi::CString;
use std::os::raw::c_char;
use utils::{read_file_to_string, string_from_c_buf};

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

    static mut nativeForkAndSpecialize: *const JNINativeMethod;
    static mut jniRegisterNativeMethods:
        *const extern "C" fn(*const JNIEnv, *const c_char, *const JNINativeMethod, i32) -> i32;
    fn nativeForkAndSpecialize_p(
        env: JNIEnv,
        clazz: JClass,
        uid: jint,
        gid: jint,
        gids: jintArray,
        runtime_flags: jint,
        rlimits: jobjectArray,
        mount_external: jint,
        se_info: jstring,
        se_name: jstring,
        fdsToClose: jintArray,
        fdsToIgnore: jintArray,
        is_child_zygote: jboolean,
        instructionSet: jstring,
        appDataDir: jstring,
    ) -> jint;
}

#[no_mangle]
extern "C" fn new_jniRegisterNativeMethods(
    env: *const JNIEnv,
    class_name: *const c_char,
    methods: *mut JNINativeMethod,
    num_methods: i32,
) -> i32 {
    trace!("jniRegisterNativeMethods");
    let class_name_string = string_from_c_buf(class_name);
    let mut new_methods: *mut JNINativeMethod = std::ptr::null_mut();

    if class_name_string == "com/android/internal/os/Zygote" {
        // 复制 methods，注入自定义逻辑
        // 因为我们无法修改原始的 methods(SEGV_ACCERR)
        new_methods =
            unsafe { libc::malloc(num_methods as usize * std::mem::size_of::<JNINativeMethod>()) }
                as *mut JNINativeMethod;
        unsafe {
            libc::memcpy(
                new_methods as *mut c_void,
                methods as *const c_void,
                std::mem::size_of::<JNINativeMethod>() * num_methods as usize,
            );
        }
        // TODO: 注入自定义代码
        debug!("class_name: {}", class_name_string);

        let methods_slice: &mut [JNINativeMethod] =
            unsafe { std::slice::from_raw_parts_mut(new_methods, num_methods as usize) };
        for mut item in methods_slice.iter_mut() {
            if string_from_c_buf(item.name) == "nativeForkAndSpecialize" {
                unsafe {
                    nativeForkAndSpecialize = item.fnPtr as *const JNINativeMethod;
                }
                debug!("method name: {}", string_from_c_buf(item.name));
                native_fork_and_specialize_hook(&mut item);
            }
        }
    }
    trace!("call raw function: {}", class_name_string);
    // 调用原始的函数
    let res = if new_methods.is_null() {
        unsafe { (*jniRegisterNativeMethods)(env, class_name, methods, num_methods) }
    } else {
        debug!("register new methods: {}", class_name_string);
        unsafe { (*jniRegisterNativeMethods)(env, class_name, new_methods, num_methods) }
    };
    if !new_methods.is_null() {
        unsafe {
            libc::free(new_methods as *mut c_void);
        }
    }
    res
}

#[ctor]
unsafe fn constructor() {
    android_log::init("paimon").unwrap();

    info!("Hello Paimon!");
    debug!("native: {:?}", nativeForkAndSpecialize);

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
    jniRegisterNativeMethods = old_func;

    if xhook_refresh(0) == 0 {
        xhook_clear();
        info!("hook installed");
    } else {
        error!("failed to refresh hook");
    }
}
