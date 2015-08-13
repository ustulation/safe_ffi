const { Cu, Ci } = require('chrome');
Cu.import("resource://gre/modules/ctypes.jsm");
Cu.import("resource://gre/modules/Services.jsm");

var sdkSelf = require('sdk/self');
// This can be expanded to show some settings page to register & unregister safe: protocol handling, probably much more
var handleIconClick = function(state) {
  var lib = ctypes.open("./libc_wrapper.so");

  var c_create_sub_directory = lib.declare('c_create_sub_directory',
                                           ctypes.default_abi,
                                           ctypes.int32_t,
                                           ctypes.char.ptr,
                                           ctypes.bool);

  var c_create_file = lib.declare('c_create_file',
                                  ctypes.default_abi,
                                  ctypes.int32_t,
                                  ctypes.char.ptr,
                                  ctypes.uint8_t.ptr,
				  ctypes.size_t);

  var c_get_file_size = lib.declare('c_get_file_size',
                                    ctypes.default_abi,
                                    ctypes.int32_t,
                                    ctypes.char.ptr,
                                    ctypes.size_t.ptr);

  var c_get_file_content = lib.declare('c_get_file_content',
                                       ctypes.default_abi,
                                       ctypes.int32_t,
                                       ctypes.char.ptr,
                                       ctypes.uint8_t.ptr);

  var c_get_file_size_from_service_home_dir = lib.declare('c_get_file_size_from_service_home_dir',
                                                          ctypes.default_abi,
                                                          ctypes.int32_t,
                                                          ctypes.char.ptr,
                                                          ctypes.char.ptr,
                                                          ctypes.char.ptr,
                                                          ctypes.bool,
                                                          ctypes.size_t.ptr);

  var c_get_file_content_from_service_home_dir = lib.declare('c_get_file_content_from_service_home_dir',
                                                             ctypes.default_abi,
                                                             ctypes.int32_t,
                                                             ctypes.char.ptr,
                                                             ctypes.char.ptr,
                                                             ctypes.char.ptr,
                                                             ctypes.bool,
                                                             ctypes.uint8_t.ptr);

  console.log("=========== Dns Test Start =============");
  console.log('Getting Home Page INDEX.html file size for "safe:www.spandan.net" ...');
  let dns_file_size = ctypes.size_t(0);
  let dns_file_size_address = dns_file_size.address();
  let error_code = c_get_file_size_from_service_home_dir("spandan.com", "www", "INDEX.html", false, dns_file_size_address);
  if (error_code == 0) {
    console.log("File size:", ctypes.cast(dns_file_size, ctypes.uint32_t).value);
    console.log("Successful !");
  } else console.log("Error-code:", error_code);

  console.log('Getting Home Page for "safe:www.spandan.net" ...');
  let DnsUint8Array_t = ctypes.ArrayType(ctypes.uint8_t, dns_file_size.value);
  let dns_file_content = DnsUint8Array_t();
  console.log("Allocated length:", dns_file_content.length);
  error_code = c_get_file_content_from_service_home_dir("spandan.com", "www", "INDEX.html", false, dns_file_content.addressOfElement(0));
  if (error_code == 0) {
    console.log("File content:", dns_file_content.readString());
    console.log("Successful !");
  } else console.log("Error-code:", error_code);
  console.log("=========== Dns Test End =============");

  console.log("                                      ");

  console.log("=========== Nfs Test Start =============");
  console.log('Creating sub-directory "/zero" ...');
  error_code = c_create_sub_directory("/zero", false);
  if (error_code == 0) console.log("Successful !");
  else console.log("Error-code:", error_code);

  // ----------------------------------------

  console.log('Creating sub-directory "/zero/one" ...');
  error_code = c_create_sub_directory("/zero/one", false);
  if (error_code == 0) console.log("Successful !");
  else console.log("Error-code:", error_code);

  // ----------------------------------------

  let js_content = "This is index.html";

  let utf8_char_array_content = ctypes.char.array()(js_content);

  // Ctors do implicit casting wherever possible
  let content_size = ctypes.size_t(utf8_char_array_content.length);

  // Assuming the html pages won't practically be > 4GiB's, the following cast is safe.
  // This is a hack as console.log does not seem to work with 64bit numbers.
  // Remove if better way found.
  let content_size_uint32_t = ctypes.cast(content_size, ctypes.uint32_t).value;

  console.log('Creating file "/zero/one/INDEX.html" with content "' + js_content + '" of size:', content_size_uint32_t, 'bytes ...');

  let ptr_utf8_char_array_content = utf8_char_array_content.address();
  let ptr_uint8_array_content = ctypes.cast(ptr_utf8_char_array_content,
		                            ctypes.uint8_t.array(content_size.value).ptr);

  error_code = c_create_file("/zero/one/INDEX.html", ptr_uint8_array_content.contents, content_size);
  if (error_code == 0) console.log("Successful !");
  else console.log("Error-code:", error_code);

  // ----------------------------------------

  console.log('Getting size for file "/zero/one/INDEX.html" ...');
  let file_size = ctypes.size_t(0);
  error_code = c_get_file_size("/zero/one/INDEX.html", file_size.address());

  // Assuming the html pages won't practically be > 4GiB's, the following cast is safe.
  // This is a hack as console.log does not seem to work with 64bit numbers.
  // Remove if better way found.
  let file_size_uint32_t = ctypes.cast(file_size, ctypes.uint32_t).value;

  if (error_code == 0) {
    console.log("File size in bytes:", file_size_uint32_t);
    console.log("Successful !");
  } else console.log("Error-code:", error_code);

  // ----------------------------------------

  console.log('Getting contents of file "/zero/one/INDEX.html" ...');
  let Uint8Array_t = ctypes.ArrayType(ctypes.uint8_t, file_size.value);
  let file_content = Uint8Array_t();
  console.log("Allocated space for content buffer in bytes:", file_content.length)
  error_code = c_get_file_content("/zero/one/INDEX.html", file_content.addressOfElement(0));
  if (error_code == 0) {
    console.log("File content:", file_content.readString());
    console.log("Successful !");
  } else console.log("Error-code:", error_code);
  console.log("=========== Nfs Test End =============")

  lib.close();
};

require('sdk/ui/button/action').ActionButton({
  id: "safe-protocol",
  label: "MaidSafe",
  icon: {
    "16": "./images/icon-16.png",
    "32": "./images/icon-32.png",
    "64": "./images/icon-64.png"
  },
  onClick: handleIconClick
});

// Register the safe: protocol handler
var factory = new require('./protocol_factory');
var handlers = require('./proto_handler');
var safeProtocol = new factory.SafeProtocol(handlers.SafeProtocolHandler);
safeProtocol.register();
