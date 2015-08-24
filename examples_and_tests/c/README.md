# C
Access the library from C.

## Build Instructions:

Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly.

Change location:
```
mkdir build
cd build
```
Build shared library `libc_wrapper.so` out of `c_wrapper.c` file:
```
gcc -shared -std=c99 -Wall -Werror -O2 -s -fvisibility=hidden -fPIC -o libc_wrapper.so ./../../../c/c_wrapper.c -L./../../../rust/target/release -lsafe_ffi
```

Build native test executable `a.out` out of `main.c` file, set library load path and run test executable:
```
gcc -std=c99 -Wall -Werror -O2 ../main.c -L. -lc_wrapper -lsodium -lm -ldl -lpthread -lrt
export LD_LIBRARY_PATH=./
./a.out
```
