# Python
Access the library from Python.

## Build Instructions:

Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly. If linker/loader cannot find libraries usual thing to do is to get current directory `./` and `/usr/local/lib` added to `LD_LIBRARY_PATH`.

Change location:
```
mkdir build
cd build
```
Use SWIG to generate the interface wrapper for python:
```
swig -python -outcurrentdir ../interface.i
```
Build shared library `libc_wrapper.so` out of `c_wrapper.c` file (libsodium needs to be compiled with fPIC enabled):
```
gcc -shared -std=c99 -Wall -Werror -O2 -s -fvisibility=hidden -fPIC -o _safe_python.so ./../../../c/c_wrapper.c ./interface_wrap.c -I/usr/include/python2.7 -L./../../../rust/target/release -lsafe_ffi -lsodium
```
Call code from python:
```
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH}./;"
python
>> import safe_python
>> safe_python.c_create_sub_directory("/zero", True)
>> safe_python.c_create_sub_directory("/zero/one", False)
>> exit()
```
// TODO this part is incomplete .. need to write more detailed documentation
