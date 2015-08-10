{
  "targets": [
    {
      "target_name": "nodejs_sample",
      "sources": [ "interface_wrap.cxx" ],
      "libraries": [
          "/home/spandan/virtualization/coding/rust-maidsafe/maidsafe-ffi-examples/safe-ffi/examples_and_tests/javascript/build/libc_wrapper.a",
          "/home/spandan/virtualization/coding/rust-maidsafe/maidsafe-ffi-examples/safe-ffi/examples_and_tests/javascript/build/libsafe_ffi.a",
          "/usr/lib/libsodium.so.13"
      ],
    }
  ]
}
