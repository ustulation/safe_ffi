# Javascript
Access the library from Javascript (node).

## Build Instructions:

Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly. If linker/loader cannot find libraries usual thing to do is to get current directory `./` and `/usr/local/lib` added to `LD_LIBRARY_PATH`.

```
mkdir build
cd build
cp ../../../rust/target/release/libsafe_ffi.a ./
gcc -c -std=c99 -Wall -Werror -fPIC ../../../c/c_wrapper.c
ar rcs libc_wrapper.a c_wrapper.o && rm c_wrapper.o
```
Modify `../binding.gyp` to point to files in this `build` directory:
```
swig -c++ -javascript -node -outcurrentdir ../interface.i
cp ../binding.gyp ./ && node-gyp configure && node-gyp build --verbose
cp ../nodejs_sample_script.js ./
nodejs nodejs_sample_script
```
