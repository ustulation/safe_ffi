# Firefox
Access the library from Firefox (js-ctypes).

## Build Instructions:

Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly. If linker/loader cannot find libraries usual thing to do is to get current directory `./` and `/usr/local/lib` added to `LD_LIBRARY_PATH`.

jpm SDK is used to build the add on. Follow this [link](https://developer.mozilla.org/en-US/Add-ons/SDK/Tools/jpm#Installation) for installation instructions.
```
mkdir build
cd build/
gcc -c -std=c99 -Wall -Werror -fPIC ../../../c/c_wrapper.c
gcc -shared -o libc_wrapper.so c_wrapper.o -L./../../../rust/target/release -lsafe_ffi -lsodium
cp libc_wrapper.so ../
cd ../
rm -rf build/
jpm run -b /usr/bin/firefox
```
Now click the safe-logo-icon to run the tests.
