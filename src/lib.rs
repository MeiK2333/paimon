#[macro_use]
extern crate ctor;
extern crate libloading;

use lazy_static::lazy_static;

#[no_mangle]
fn process() {
    println!("process");
}

#[ctor]
fn constructor() {
    println!("constructor");
}

#[dtor]
fn shutdown() {
    eprintln!("shutdown");
}

lazy_static! {
    static ref LIB: libloading::Library = libloading::Library::new("/system/lib64").unwrap();
}

macro_rules! func_def {
    ($func_name:ident, $func_return:ty) => {
        #[no_mangle]
        unsafe fn $func_name() -> $func_return {
            let func: libloading::Symbol<unsafe extern "C" fn() -> $func_return> =
                LIB.get(stringify!($func_name).as_bytes()).unwrap();
            func()
        }
    };
    ($func_name:ident, $func_return:ty, $($arg: ident: $param:ty),*) => {
        #[no_mangle]
        unsafe fn $func_name($($arg: $param),*) -> $func_return {
            let func: libloading::Symbol<unsafe extern "C" fn($($param),*) -> $func_return> =
                LIB.get(stringify!($func_name).as_bytes()).unwrap();
            func($($arg),*)
        }
    };
}

#[no_mangle]
#[allow(non_camel_case_types)]
struct memtrack_proc {}

// #[no_mangle]
// unsafe fn memtrack_init() -> i32 {
//     let func: libloading::Symbol<unsafe extern "C" fn() -> i32> =
//         LIB.get("memtrack_init".as_bytes()).unwrap();
//     func()
// }
func_def!(memtrack_init, i32);

func_def!(memtrack_proc_destroy, ());

func_def!(memtrack_proc_gl_pss, i32, arg: *const memtrack_proc);

// #[no_mangle]
// unsafe fn memtrack_proc_get(arg1: *const memtrack_proc, arg2: i32) -> i32 {
//     let func: libloading::Symbol<unsafe extern "C" fn(*const memtrack_proc, i32) -> i32> =
//         LIB.get("memtrack_proc_get".as_bytes()).unwrap();
//     func(arg1, arg2)
// }
func_def!(
    memtrack_proc_get,
    i32,
    arg1: *const memtrack_proc,
    arg2: i32
);

func_def!(memtrack_proc_gl_pss_mapped, i64, arg: *const memtrack_proc);

func_def!(memtrack_proc_gl_total, i64, arg: *const memtrack_proc);

func_def!(memtrack_proc_graphics_pss, i64, arg: *const memtrack_proc);

func_def!(
    memtrack_proc_graphics_pss_mapped,
    i64,
    arg: *const memtrack_proc
);

func_def!(memtrack_proc_graphics_total, i64, arg: *const memtrack_proc);

func_def!(memtrack_proc_new, i64, arg: *const memtrack_proc);

func_def!(memtrack_proc_other_pss, i64, arg: *const memtrack_proc);

func_def!(
    memtrack_proc_other_pss_mapped,
    i64,
    arg: *const memtrack_proc
);

func_def!(memtrack_proc_other_total, i64, arg: *const memtrack_proc);

func_def!(memtrack_proc_multimedia_pss, i64, arg: *const memtrack_proc);

func_def!(
    memtrack_proc_multimedia_pss_mapped,
    i64,
    arg: *const memtrack_proc
);

func_def!(
    memtrack_proc_multimedia_total,
    i64,
    arg: *const memtrack_proc
);

func_def!(memtrack_proc_camera_pss, i64, arg: *const memtrack_proc);

func_def!(
    memtrack_proc_camera_pss_mapped,
    i64,
    arg: *const memtrack_proc
);

func_def!(memtrack_proc_camera_total, i64, arg: *const memtrack_proc);
