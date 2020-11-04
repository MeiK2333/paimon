#include <jni.h>

extern "C" {
JNINativeMethod *nativeForkAndSpecialize = NULL;
int (*jniRegisterNativeMethods)(JNIEnv *env, const char *className,
                                const JNINativeMethod *methods,
                                int numMethods) = NULL;

using nativeForkAndSpecialize_p_t = jint(JNIEnv *, jclass, jint, jint,
                                         jintArray, jint, jobjectArray, jint,
                                         jstring, jstring, jintArray, jintArray,
                                         jboolean, jstring, jstring);

jint nativeForkAndSpecialize_p(JNIEnv *env, jclass clazz, jint uid, jint gid,
                               jintArray gids, jint runtime_flags,
                               jobjectArray rlimits, jint mount_external,
                               jstring se_info, jstring se_name,
                               jintArray fdsToClose, jintArray fdsToIgnore,
                               jboolean is_child_zygote, jstring instructionSet,
                               jstring appDataDir) {
    jint res = ((nativeForkAndSpecialize_p_t *)nativeForkAndSpecialize)(
        env, clazz, uid, gid, gids, runtime_flags, rlimits, mount_external,
        se_info, se_name, fdsToClose, fdsToIgnore, is_child_zygote,
        instructionSet, appDataDir);

    return res;
}
}
