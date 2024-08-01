# for listening TCP SmartSocket commands start server example before run dyn_lib
from ctypes import *

get_my_integer_lib = cdll.LoadLibrary("../target/debug/smart_home_dyn_lib.dll")

print(get_my_integer_lib.get_integer())