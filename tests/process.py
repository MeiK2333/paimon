from ctypes import cdll

lib = cdll.LoadLibrary("target/debug/libpaimon.so")
lib.process()
