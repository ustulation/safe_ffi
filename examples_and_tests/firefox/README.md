# Firefox
Access the library from Firefox (js-ctypes).

## Build Instructions:

Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly. If linker/loader cannot find libraries usual thing to do is to get current directory `./` and `/usr/local/lib` added to `LD_LIBRARY_PATH`.

jpm SDK is used to build the add on. Follow this [link](https://developer.mozilla.org/en-US/Add-ons/SDK/Tools/jpm#Installation) for installation instructions.
```
mkdir build
cd build/
gcc -shared -std=c99 -Wall -Werror -O2 -s -fvisibility=hidden -fPIC -o libc_wrapper.so ./../../../c/c_wrapper.c -L./../../../rust/target/release -lsafe_ffi -lsodium
cp libc_wrapper.so ../
cd ../
rm -rf build/
jpm run -b /usr/bin/firefox
```
Now click the safe-logo-icon to run the tests.

### For Windows:
The problem with Windows is that Firefox does not yet (as i write this) support a 64-bit version. So libraries need to be compiled with 32 bit Rust and MinGW. 32 bit Rust however has problems compiling/downloading packages (during the time this was written) which also manifests as failing `Appveyor` builds for other crates. Then there is a 64-bit Developers' Edition for Firefox which can be downloaded and works fine with the 64-bit shared-objects. For this test 64-bit for Windows under `English(British)` was downloaded from [link-to-64-bit-Firefox](https://www.mozilla.org/en-US/firefox/developer/all/).

**Edit:**  As of today, 26-Aug-2015, with the updates of dependencies like `sodiumoxide` in [crates.io](https://crates.io/) and with latest nightly of 32-bit rust-compiler suite, building of 32-bit libraries is fine and works with the 32-bit Firefox. So modifying the following to support 32-bit yeilds identical (working) results to those for 64-bit platforms.

(mingw-w64 was used).

The commands were run from git-bash prompt, thus its similarity to Linux prompt. For normal `cmd` prompt you will need to make a few changes (eg., `mkdir` would become `md` etc).
```
mkdir build
cd build/
gcc -shared -std=c99 -Wall -Werror -O2 -s -fvisibility=hidden -o libc_wrapper.dll ./../../../c/c_wrapper.c -L./../../../rust/target/release -lsafe_ffi -L./path/to/native-sodium -lsodium -lIphlpapi -lws2_32 -luserenv -Wl,--out-implib,libc_wrapper.dll.a
cp libc_wrapper.dll ../
cd ../
rm -rf build/
jpm run -b /c-or-d-etc/path/to/64-bit/firefox.exe
```

## Note
Ofcourse modify `index.js` to point to `"./libc_wrapper.dll"` or `"./libc_wrapper.so"` according to the build-platform. Also `libsodium` and other native dependencies must be compiled for 32-bit platform in order to work with 32-bit builds.
