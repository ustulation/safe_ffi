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

//extern crate cbor;
//extern crate routing;
extern crate safe_nfs;
//extern crate sodiumoxide;
#[macro_use] extern crate safe_client;

/// Errors during FFI operations
pub enum FfiError {
    /// Errors from safe_client
    ClientError(safe_client::errors::ClientError),
    /// Errors from safe_nfs
    NfsError(safe_nfs::errors::NfsError),
    /// Invalid Path given
    CouldNotDecodePath,
    /// Unexpected or some programming error
    Unexpected(String),
}

impl std::fmt::Debug for FfiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            FfiError::ClientError(ref error) => write!(f, "FfiError::ClientError -> {:?}", error),
            FfiError::NfsError(ref error)    => write!(f, "FfiError::NfsError -> {:?}", error),
            FfiError::CouldNotDecodePath     => write!(f, "FfiError::CouldNotDecodePath"),
            FfiError::Unexpected(ref error)  => write!(f, "FfiError::Unexpected::{{{:?}}}", error),
        }
    }
}

/// Tokenise the give path
pub fn path_tokeniser(path: &String) -> Result<Vec<String>, FfiError> {
    Ok(path.split("/").filter(|a| !a.is_empty()).map(|a| a.to_string()).collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_path() {
        let path_0 = "/abc/d/ef".to_string();
        let path_1 = "/abc/d/ef/".to_string();
        let path_2 = "///abc///d/ef////".to_string();

        let expected = vec!["abc".to_string(),
                            "d".to_string(),
                            "ef".to_string()];

        let tokenised_0 = eval_result!(path_tokeniser(&path_0));
        let tokenised_1 = eval_result!(path_tokeniser(&path_1));
        let tokenised_2 = eval_result!(path_tokeniser(&path_2));

        assert_eq!(tokenised_0, expected);
        assert_eq!(tokenised_1, expected);
        assert_eq!(tokenised_2, expected);
    }
}
