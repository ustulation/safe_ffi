# Qt
Access the library from Qt

## Build Instructions:

Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly. If linker/loader cannot find libraries usual thing to do is to get current directory `./` and `/usr/local/lib` added to `LD_LIBRARY_PATH`.

```
mkdir build
cd build
gcc -c -std=c99 -Wall -Werror -fPIC ../../../c/c_wrapper.c 
ar rcs libc_wrapper.a c_wrapper.o && rm c_wrapper.o
cd ../
```
Modify the Qt `.pro` file to correct directories as far as absolutely paths are concerned. Open the `.pro` file in Qt-Creator and make sure to point to `./build` directory (created above) in the configure wizard for qt to create further sub-direcories inside for Release and Debug builds. Run the app.
