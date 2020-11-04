fn main() {
    cc::Build::new()
        .file("external/xhook/xh_core.c")
        .file("external/xhook/xh_elf.c")
        .file("external/xhook/xh_jni.c")
        .file("external/xhook/xh_log.c")
        .file("external/xhook/xh_util.c")
        .file("external/xhook/xh_version.c")
        .file("external/xhook/xhook.c")
        .file("src/c/utils.cc")
        .compile("xhook");
}
