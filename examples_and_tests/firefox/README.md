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

### For Windows:
The problem with Windows is that Firefox does not yet (as i write this) support a 64-bit version. So libraries need to be compiled with 32 bit Rust and MinGW. 32 bit Rust however has problems compiling/downloading packages (during the time this was written) which also manifests as failing `Appveyor` builds for other crates. Then there is a 64-bit Developers' Edition for Firefox which can be downloaded and works fine with the 64-bit shared-objects. For this test 64-bit for Windows under `English(British)` was downloaded from [link-to-64-bit-Firefox](https://www.mozilla.org/en-US/firefox/developer/all/).

(mingw-w64 was used).

The commands were run from git-bash prompt, thus its similarity to Linux prompt. For normal `cmd` prompt you will need to make a few changes (eg., `mkdir` would become `md` etc).
```
mkdir build
cd build/
gcc -c -std=c99 -Wall -Werror ../../../c/c_wrapper.c
gcc -shared -O2 -s -o libc_wrapper.dll c_wrapper.o -L./../../../rust/target/release -lsafe_ffi -L./path/to/native-sodium -lsodium -lIphlpapi -lws2_32 -luserenv -Wl,--out-implib,libc_wrapper.dll.a
cp libc_wrapper.dll ../
cd ../
rm -rf build/
jpm run -b /c-or-d-etc/path/to/64-bit/firefox.exe
```

## Note
Ofcourse modify `index.js` to point to `"./libc_wrapper.dll"` or `"./libc_wrapper.so"` according to the build-platform.
