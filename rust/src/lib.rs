// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

#![crate_name = "nfs_ffi"]
#![crate_type = "lib"]
// TODO Change this
#![doc(html_logo_url = "http://maidsafe.net/img/Resources/branding/maidsafe_logo.fab2.png",
       html_favicon_url = "http://maidsafe.net/img/favicon.ico",
              html_root_url = "http://dirvine.github.io/dirvine/safe_dns/")]

///////////////////////////////////////////////////
//               LINT
///////////////////////////////////////////////////

#![forbid(bad_style, warnings)]

#![deny(deprecated, improper_ctypes, missing_docs, non_shorthand_field_patterns,
overflowing_literals, plugin_as_library, private_no_mangle_fns, private_no_mangle_statics,
raw_pointer_derive, stable_features, unconditional_recursion, unknown_lints,
unsigned_negation, unused, unused_allocation, unused_attributes, unused_comparisons,
unused_features, unused_parens, while_true)]

#![warn(trivial_casts, trivial_numeric_casts, unused_extern_crates, unused_import_braces,
unused_qualifications, variant_size_differences)]

///////////////////////////////////////////////////

// TODO
//! #Nfs-FFI Library
//! [Project github page](https://github.com/maidsafe/safe_dns)

extern crate libc;
extern crate safe_nfs;
#[macro_use] extern crate safe_client;

use std::error::Error;

#[macro_use] mod macros;

mod errors;
mod implementation;

/// Create a subdirectory. The Name of the subdirectory is the final token in the given path. Eg.,
/// if given path = "/a/b/c/d" then "d" is interpreted as the subdirectory intended to be created.
#[no_mangle]
pub extern fn create_sub_directory(c_path: *const libc::c_char, is_private: bool) -> i32 {
    let cstr_path = unsafe { std::ffi::CStr::from_ptr(c_path) };
    let mut tokens = ffi_try!(implementation::path_tokeniser(cstr_path));

    let sub_dir_name = ffi_try!(tokens.pop().ok_or(errors::FfiError::InvalidPath));
    let mut parent_dir_listing = ffi_try!(implementation::get_final_subdirectory(&tokens));
    let dir_helper = safe_nfs::helper::directory_helper::DirectoryHelper::new(((implementation::get_test_client())));

    let access_level = if is_private {
        safe_nfs::AccessLevel::Private
    } else {
        safe_nfs::AccessLevel::Public
    };

    let _ = ffi_try!(dir_helper.create(sub_dir_name,
                                       safe_nfs::UNVERSIONED_DIRECTORY_LISTING_TAG,
                                       vec![],
                                       false,
                                       access_level,
                                       Some(&mut parent_dir_listing)));

    0
}

/// Create a file. The Name of the file is the final token in the given path. Eg.,
/// if given path = "/a/b/c/d" then "d" is interpreted as the file intended to be created.
#[no_mangle]
pub extern fn create_file(c_path: *const libc::c_char, c_content: *const libc::c_char) -> i32 {
    let cstr_path = unsafe { std::ffi::CStr::from_ptr(c_path) };
    let mut tokens = ffi_try!(implementation::path_tokeniser(cstr_path));

    let file_name = ffi_try!(tokens.pop().ok_or(errors::FfiError::InvalidPath));
    let parent_dir_listing = ffi_try!(implementation::get_final_subdirectory(&tokens));
    let file_helper = safe_nfs::helper::file_helper::FileHelper::new(implementation::get_test_client());

    let mut writer = ffi_try!(file_helper.create(file_name,
                                                 vec![],
                                                 parent_dir_listing));

    let cstr_content = unsafe { std::ffi::CStr::from_ptr(c_content) };
    writer.write(cstr_content.to_bytes(), 0);
    let _ = ffi_try!(writer.close());

    0
}

/// Get the size of the file. c_size should be properly and sufficiently pre-allocated.
#[no_mangle]
pub extern fn get_file_size(c_path: *const libc::c_char, c_size: *mut libc::c_int) -> i32 {
    let cstr_path = unsafe { std::ffi::CStr::from_ptr(c_path) };
    let mut tokens = ffi_try!(implementation::path_tokeniser(cstr_path));

    let file_name = ffi_try!(tokens.pop().ok_or(errors::FfiError::InvalidPath));
    let parent_dir_listing = ffi_try!(implementation::get_final_subdirectory(&tokens));

    let size = ffi_try!(implementation::get_file_size(&file_name, &parent_dir_listing));

    unsafe { std::ptr::write(c_size, size as libc::c_int) };

    0
}

/// Read a file. The Name of the file is the final token in the given path. Eg.,
/// if given path = "/a/b/c/d" then "d" is interpreted as the file intended to be read.
/// c_content_buf should be properly and sufficiently pre-allocated.
#[no_mangle]
pub extern fn get_file_content(c_path: *const libc::c_char, c_content_buf: *mut libc::c_char) -> i32 {
    let cstr_path = unsafe { std::ffi::CStr::from_ptr(c_path) };
    let mut tokens = ffi_try!(implementation::path_tokeniser(cstr_path));

    let file_name = ffi_try!(tokens.pop().ok_or(errors::FfiError::InvalidPath));
    let parent_dir_listing = ffi_try!(implementation::get_final_subdirectory(&tokens));
    let data_vec = ffi_try!(implementation::get_file_content(&file_name, &parent_dir_listing));

    let cstring_content = ffi_try!(std::ffi::CString::new(data_vec).map_err(|error| errors::FfiError::from(error.description())));
    unsafe { std::ptr::copy(cstring_content.as_ptr(), c_content_buf, cstring_content.as_bytes_with_nul().len()) };

    0
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn create_directories_files_and_read_files() {
        let size_of_c_int = ::std::mem::size_of::<::libc::c_int>();
        let size_of_c_char = ::std::mem::size_of::<::libc::c_char>();

        // --------------------------------------------------------------------
        // Create Sub-directory /a - c string size with \0 = 3
        // --------------------------------------------------------------------
        let mut c_path = unsafe { ::libc::malloc(3 * size_of_c_char as ::libc::size_t) } as *mut ::libc::c_char;

        {
            let cstring_path = eval_result!(::std::ffi::CString::new("/a").map_err(|error| ::errors::FfiError::from(error.description())));

            let path_lenght_for_c = cstring_path.as_bytes_with_nul().len();
            assert_eq!(path_lenght_for_c, 3 * size_of_c_char);

            unsafe { ::std::ptr::copy(cstring_path.as_ptr(), c_path, path_lenght_for_c) };
        }

        assert_eq!(create_sub_directory(c_path, true), 0); // TODO passing false fails in nfs-crate - UnsuccessfulEncodeDecode
        unsafe { ::libc::free(c_path as *mut ::libc::c_void) };

        // --------------------------------------------------------------------
        // Create Sub-directory /a/last - c string size with \0 = 8
        // --------------------------------------------------------------------
        c_path = unsafe { ::libc::malloc(8 * size_of_c_char as ::libc::size_t) } as *mut ::libc::c_char;

        {
            let cstring_path = eval_result!(::std::ffi::CString::new("/a/last").map_err(|error| ::errors::FfiError::from(error.description())));

            let path_lenght_for_c = cstring_path.as_bytes_with_nul().len();
            assert_eq!(path_lenght_for_c, 8 * size_of_c_char);

            unsafe { ::std::ptr::copy(cstring_path.as_ptr(), c_path, path_lenght_for_c) };
        }

        assert_eq!(create_sub_directory(c_path, true), 0); // TODO passing false fails in nfs-crate - UnsuccessfulEncodeDecode
        unsafe { ::libc::free(c_path as *mut ::libc::c_void) };

        // --------------------------------------------------------------------
        // Create file /a/last/file.txt - c string size with \0 = 17
        // --------------------------------------------------------------------
        c_path = unsafe { ::libc::malloc(17 * size_of_c_char as ::libc::size_t) } as *mut ::libc::c_char;

        let cstring_content = eval_result!(::std::ffi::CString::new("This is the file content.").map_err(|error| ::errors::FfiError::from(error.description())));

        {
            let cstring_path = eval_result!(::std::ffi::CString::new("/a/last/file.txt").map_err(|error| ::errors::FfiError::from(error.description())));

            let path_lenght_for_c = cstring_path.as_bytes_with_nul().len();
            assert_eq!(path_lenght_for_c, 17 * size_of_c_char);

            unsafe { ::std::ptr::copy(cstring_path.as_ptr(), c_path, path_lenght_for_c) };
        }

        assert_eq!(create_file(c_path, cstring_content.as_ptr()), 0);
        unsafe { ::libc::free(c_path as *mut ::libc::c_void) };

        // --------------------------------------------------------------------
        // Get the size of the file
        // --------------------------------------------------------------------
        c_path = unsafe { ::libc::malloc(17 * size_of_c_char as ::libc::size_t) } as *mut ::libc::c_char;

        {
            let cstring_path = eval_result!(::std::ffi::CString::new("/a/last/file.txt").map_err(|error| ::errors::FfiError::from(error.description())));

            let path_lenght_for_c = cstring_path.as_bytes_with_nul().len();
            assert_eq!(path_lenght_for_c, 17 * size_of_c_char);

            unsafe { ::std::ptr::copy(cstring_path.as_ptr(), c_path, path_lenght_for_c) };
        }

        let c_size = unsafe { ::libc::malloc(1 * size_of_c_int as ::libc::size_t) } as *mut ::libc::c_int;

        assert_eq!(get_file_size(c_path, c_size), 0);
        unsafe { assert_eq!(*c_size as usize, cstring_content.as_bytes().len()) };

        unsafe { ::libc::free(c_path as *mut ::libc::c_void) };

        // --------------------------------------------------------------------
        // Get the contents of the file
        // --------------------------------------------------------------------
        c_path = unsafe { ::libc::malloc(17 * size_of_c_char as ::libc::size_t) } as *mut ::libc::c_char;

        {
            let cstring_path = eval_result!(::std::ffi::CString::new("/a/last/file.txt").map_err(|error| ::errors::FfiError::from(error.description())));

            let path_lenght_for_c = cstring_path.as_bytes_with_nul().len();
            assert_eq!(path_lenght_for_c, 17 * size_of_c_char);

            unsafe { ::std::ptr::copy(cstring_path.as_ptr(), c_path, path_lenght_for_c) };
        }

        let c_content = unsafe { ::libc::malloc(((*c_size + 1) as usize * ::std::mem::size_of::<::libc::c_int>()) as ::libc::size_t) } as *mut ::libc::c_char;

        assert_eq!(get_file_content(c_path, c_content), 0);

        let read_cstr_content = unsafe { ::std::ffi::CStr::from_ptr(c_content) };
        assert_eq!(&*cstring_content, read_cstr_content);

        unsafe { ::libc::free(c_path as *mut ::libc::c_void) };
        unsafe { ::libc::free(c_size as *mut ::libc::c_void) };
        unsafe { ::libc::free(c_content as *mut ::libc::c_void) };
    }
}
