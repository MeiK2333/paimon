use crate::utils::string_from_c_buf;
use crate::{nativeForkAndSpecialize, nativeForkAndSpecialize_p};
use jni::sys::{jboolean, jclass, jint, jintArray, jobjectArray, jstring, JNIEnv, JNINativeMethod};
use libc::c_void;

#[no_mangle]
pub extern "system" fn new_native_fork_and_specialize(
    env: *const JNIEnv,
    clazz: jclass,
    uid: jint,
    gid: jint,
    gids: jintArray,
    runtime_flags: jint,
    rlimits: jobjectArray,
    mount_external: jint,
    se_info: jstring,
    se_name: jstring,
    fds_to_close: jintArray,
    fds_to_ignore: jintArray,
    is_child_zygote: jboolean,
    instruction_set: jstring,
    app_data_dir: jstring,
) -> jint {
    native_fork_and_specialize_pre();

    let res = unsafe {
        nativeForkAndSpecialize_p(
            env,
            clazz,
            uid,
            gid,
            gids,
            runtime_flags,
            rlimits,
            mount_external,
            se_info,
            se_name,
            fds_to_close,
            fds_to_ignore,
            is_child_zygote,
            instruction_set,
            app_data_dir,
        )
    };
    native_fork_and_specialize_post();
    res
}

#[no_mangle]
pub extern "C" fn native_fork_and_specialize_hook(method: &mut JNINativeMethod) {
    let signature = string_from_c_buf(method.signature);
    // debug!("method signature: {}", signature);
    // 暂时只适配 Android 9
    if signature != "(II[II[[IILjava/lang/String;Ljava/lang/String;[I[IZLjava/lang/String;Ljava/lang/String;)I" {
        return;
    }
    unsafe {
        nativeForkAndSpecialize = method.fnPtr as *const JNINativeMethod;
    }
    method.fnPtr = new_native_fork_and_specialize as *mut c_void;
}

fn native_fork_and_specialize_pre() {
    debug!("native_fork_and_specialize_pre");
}

fn native_fork_and_specialize_post() {
    debug!("native_fork_and_specialize_post");
}
