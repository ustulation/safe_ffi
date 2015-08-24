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

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef _WIN32
#    ifndef _WIN64
#        define CALLING_CONVENTION __cdecl
#    else
#        define CALLING_CONVENTION
#    endif
#    ifndef IMPORT_LIB
#        define SYMBOL_VISIBLE __declspec(dllexport)
#    else
#        define SYMBOL_VISIBLE __declspec(dllimport)
#    endif
#else
#    define CALLING_CONVENTION
#    define SYMBOL_VISIBLE __attribute__((visibility ("default")))
#endif

#ifdef __cplusplus
extern "C" {
#endif

SYMBOL_VISIBLE int32_t CALLING_CONVENTION c_create_sub_directory(const char* absolute_path,
                                                                 bool is_private);

SYMBOL_VISIBLE int32_t CALLING_CONVENTION c_create_file(const char* absolute_path,
                                                        const uint8_t* file_content,
                                                        const size_t content_size);

SYMBOL_VISIBLE int32_t CALLING_CONVENTION c_get_file_size(const char* absolute_path,
                                                          size_t* obtain_file_size);

SYMBOL_VISIBLE int32_t CALLING_CONVENTION c_get_file_content(const char* absolute_path,
                                                             uint8_t* obtain_file_content);

SYMBOL_VISIBLE int32_t CALLING_CONVENTION c_register_dns(const char* long_name,
                                                         const char* service_name,
                                                         const char* abs_path_to_service_home_dir);

SYMBOL_VISIBLE int32_t CALLING_CONVENTION c_add_service(const char* long_name,
                                                        const char* service_name,
                                                        const char* abs_path_to_service_home_dir);

SYMBOL_VISIBLE int32_t CALLING_CONVENTION c_get_file_size_from_service_home_dir(const char* long_name,
                                                                                const char* service_name,
                                                                                const char* file_name,
                                                                                bool is_private,
                                                                                size_t* obtain_file_size);

SYMBOL_VISIBLE int32_t CALLING_CONVENTION c_get_file_content_from_service_home_dir(const char* long_name,
                                                                                   const char* service_name,
                                                                                   const char* file_name,
                                                                                   bool is_private,
                                                                                   uint8_t* obtain_file_content);
#ifdef __cplusplus
}
#endif

