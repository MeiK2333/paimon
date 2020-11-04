compiled=$(cargo ndk --platform 21 --target i686-linux-android build --release)
if [ $? != 0 ]; then
    echo "compile error"
    exit 1
fi
adb root
sleep 2
adb remount
sleep 2
adb push target/i686-linux-android/release/libpaimon.so /system/lib/libpaimon.so
sleep 1
exists=$(adb shell "cat /system/etc/public.libraries.txt | grep libpaimon.so")
if [ $? != 0 ]; then
    echo "echo libpaimon.so to /system/etc/public.libraries.txt"
    adb shell "echo 'libpaimon.so' >> /system/etc/public.libraries.txt"
    sleep 1
fi
adb reboot
