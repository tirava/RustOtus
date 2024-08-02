# for listening TCP SmartSocket commands start server example before run dyn_lib
from ctypes import *

my_lib = cdll.LoadLibrary("../target/debug/smart_home_dyn_lib.dll")

send_command = my_lib.send_command
send_command.restype = c_char_p
send_command.argtypes = [c_char_p, c_char_p]

result = send_command(c_char_p(b"127.0.0.1:54321"), c_char_p(b"info"))
print(str(result, 'utf8'))

# print(send_command("on"))
# print(send_command("info"))

# print(send_command("off"))
# print(send_command("info"))

# print(send_command("qqq"))
