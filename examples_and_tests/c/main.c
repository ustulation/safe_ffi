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

#include "../../c/c_wrapper.h"

#include <stdio.h>
#include <assert.h>
#include <string.h>
#include <stdlib.h>

int main() {
    {
        printf("\n----------------------------------------------\n");
        printf(  "            BEGIN NFS TEST                    \n");
        printf(  "----------------------------------------------\n");
        printf("\nCreating path \"/a/\" ...\n");
        const char* absolute_path_a = "/a";
        int32_t error_code = c_create_sub_directory(absolute_path_a, false);
        assert(!error_code);
        printf("Successful !\n");

        printf("\nCreating path \"/a/last/\" ...\n");
        const char* absolute_path_a_last = "/a/last/";
        error_code = c_create_sub_directory(absolute_path_a_last, false);
        assert(!error_code);
        printf("Successful !\n");

        const char* file_content = "This is the file's content.";
        printf("\nCreating file \"/a/last/hello.txt\" with content: \"%s\" of size: %zd Bytes...\n", file_content, strlen(file_content));
        const char* absolute_path_a_last_hello = "/a/last/hello.txt";
        error_code = c_create_file(absolute_path_a_last_hello, file_content);
        assert(!error_code);
        printf("Successful !\n");

        printf("\nGetting file size for \"/a/last/hello.txt\" ...\n");
        int file_size = -1;
        int* p_file_size = &file_size;
        assert(strlen(file_content) != file_size);
        error_code = c_get_file_size(absolute_path_a_last_hello, p_file_size);
        assert(!error_code);
        assert(strlen(file_content) == file_size);
        printf("Obtained size: %d Bytes\n", file_size);
        printf("Successful !\n");

        printf("\nGetting file content for \"/a/last/hello.txt\" ...\n");
        char* content_buffer = malloc(file_size + 1);
        error_code = c_get_file_content(absolute_path_a_last_hello, content_buffer);
        assert(!error_code);
        assert(!strcmp(content_buffer, file_content));
        printf("Obtained data: \"%s\"\n", content_buffer);
        printf("Successful !\n");

        printf("\nFreeing heap ...\n");
        free(content_buffer);
        printf("Successful !\n");
    }

    {
        printf("\n----------------------------------------------\n");
        printf(  "            BEGIN DNS TEST                    \n");
        printf(  "----------------------------------------------\n");
        printf("\nCreating path \"/SomeDir/\" ...\n");
        const char* absolute_path_some_dir = "/SomeDir";
        int32_t error_code = c_create_sub_directory(absolute_path_some_dir, false);
        assert(!error_code);
        printf("Successful !\n");

        printf("\nCreating path \"/SomeDir/service-www/\" ...\n");
        const char* absolute_path_some_dir_www = "/SomeDir/service-www/";
        error_code = c_create_sub_directory(absolute_path_some_dir_www, false);
        assert(!error_code);
        printf("Successful !\n");

        const char* file_content_www = "This is the Homepage for Service \"www\".";
        printf("\nCreating file \"/SomeDir/service-www/HOME.html\" with content: \"%s\" of size: %zd Bytes...\n", file_content_www, strlen(file_content_www));
        const char* absolute_path_some_dir_www_homefile = "/SomeDir/service-www/HOME.html";
        error_code = c_create_file(absolute_path_some_dir_www_homefile, file_content_www);
        assert(!error_code);
        printf("Successful !\n");

        printf("\nCreating path \"/SomeDir/service-blog/\" ...\n");
        const char* absolute_path_some_dir_blog = "/SomeDir/service-blog/";
        error_code = c_create_sub_directory(absolute_path_some_dir_blog, false);
        assert(!error_code);
        printf("Successful !\n");

        const char* file_content_blog = "This is the Homepage for Service \"blog\".";
        printf("\nCreating file \"/SomeDir/service-blog/HOME.html\" with content: \"%s\" of size: %zd Bytes...\n", file_content_blog, strlen(file_content_blog));
        const char* absolute_path_some_dir_blog_homefile = "/SomeDir/service-blog/HOME.html";
        error_code = c_create_file(absolute_path_some_dir_blog_homefile, file_content_blog);
        assert(!error_code);
        printf("Successful !\n");

        printf("\nRegistering DNS \"maidsafe.net\" with service \"www\" ...\n");
        const char* long_name = "maidsafe.net";
        const char* service_www = "www";
        error_code = c_register_dns(long_name, service_www, absolute_path_some_dir_www);
        assert(!error_code);
        printf("Successful !\n");

        printf("\nAdd service \"blog\" to registered DNS \"maidsafe.net\" ...\n");
        const char* service_blog = "blog";
        error_code = c_add_service(long_name, service_blog, absolute_path_some_dir_blog);
        assert(!error_code);
        printf("Successful !\n");

        printf("\nGetting Home Page for Service \"%s\" of Dns \"%s\" ...\n", service_www, long_name);
        char* content_buffer = malloc(strlen(file_content_www) + 1);
        error_code = c_get_file_content_from_service_home_dir(long_name, service_www, "HOME.html", false, content_buffer);
        assert(!error_code);
        assert(!strcmp(content_buffer, file_content_www));
        printf("Obtained data: \"%s\"\n", content_buffer);
        printf("Successful !\n");

        printf("\nFreeing heap ...\n");
        free(content_buffer);
        printf("Successful !\n");

        printf("\nGetting Home Page for Service \"%s\" of Dns \"%s\" ...\n", service_blog, long_name);
        content_buffer = malloc(strlen(file_content_blog) + 1);
        error_code = c_get_file_content_from_service_home_dir(long_name, service_blog, "HOME.html", false, content_buffer);
        assert(!error_code);
        assert(!strcmp(content_buffer, file_content_blog));
        assert(strcmp(content_buffer, file_content_www));
        printf("Obtained data: \"%s\"\n", content_buffer);
        printf("Successful !\n");

        printf("\nFreeing heap ...\n");
        free(content_buffer);
        printf("Successful !\n");
    }

    printf("\nExiting ...\n");
    return 0;
}
