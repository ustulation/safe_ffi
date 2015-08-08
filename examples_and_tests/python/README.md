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
gcc -c -std=c99 -Wall -Werror -fPIC ../../../c/c_wrapper.c ./interface_wrap.c -I/usr/include/python2.7
gcc -shared -o _python_sample.so c_wrapper.o interface_wrap.o -L./../../../rust/target/release -lsafe_ffi -lsodium
```
Call code from python:
```
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH}./;"
python
import python_sample
python_sample.c_create_sub_directory("/zero", True)
```
// TODO this part is incomplete .. need to write more detailed documentation
