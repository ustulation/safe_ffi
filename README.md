# safe_ffi
This repository provides FFI bindings for the Client modules.

**IMPORTANT [01-Sep-2015]:** The *FFI* contents of this repository has been replicated into the newly created [official MaidSafe repository](https://github.com/maidsafe/safe_ffi). It will be more actively maintained there. This repository will stay mostly as a reference on how to use the FFI-bindings through C-indirection and/or [SWIG](http://www.swig.org/) for target languages/browsers. As such it is not intended that everything noted here compiles although it does right now. Any PR's to to improve, correct or expand will however be monitored and accepted.

## Build Instructions:

Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly.

Build static library `libsafe_ffi.a` out of `safe_ffi` crate:
```
cd rust/
cargo build --release
```
To build against Mock-routing (Simulates routing and vault minutely without networking - only used for testing purposes) do:
```
cd rust/
cargo build --release --features "use-mock-routing"
```
This will give `rust/target/release/libsafe_ffi.a`
Change location to desired programming language under `examples_and_tests` and follow the instructions there.

## Note:
There is no difference between using rust-compiled `.so/.a/.dll`'s or C `.so/.a/.dll`'s. The C indirection is only for those who would want to use `SWIG` and also for demonstration purpose. In examples that don't use `SWIG`, the rust-compiled binaries (from `safe_ffi`) can directly be used.
