# safe_ffi
This repository provides FFI bindings for safe_nfs and safe_dns.

## Build Instructions:

Instructions are tested for Ubuntu-Linux. Modify for other platforms accordingly.

Build static library `libsafe_ffi.a` out of `safe_ffi` crate:
```
cd rust/
cargo build --release
```
This will give `rust/target/release/libsafe_ffi.a`
Change location to desired programming language under examples_and_tests and follow the instructions there.
