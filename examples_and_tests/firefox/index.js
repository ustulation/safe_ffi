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

  console.log("=========== Test Start =============");
  console.log('Creating sub-directory "/zero" ...');
  var error_code = c_create_sub_directory("/zero", false);
  if (error_code == 0) console.log("Successful !");
  else console.log("Error-code:", error_code);

  console.log('Creating sub-directory "/zero/one" ...');
  error_code = c_create_sub_directory("/zero/one", false);
  if (error_code == 0) console.log("Successful !");
  else console.log("Error-code:", error_code);

  console.log('Creating file "/zero/one/INDEX.html" with content "This is index.html" ...');
  let js_content = "This is index.html";

  let utf8_char_array_content = ctypes.char.array()(js_content);
  let content_size = ctypes.size_t(utf8_char_array_content.length);

  let ptr_utf8_char_array_content = utf8_char_array_content.address();
  let ptr_uint8_array_content = ctypes.cast(ptr_utf8_char_array_content,
		                            ctypes.uint8_t.array(utf8_char_array_content.length).ptr);

  error_code = c_create_file("/zero/one/INDEX.html", ptr_uint8_array_content.contents, content_size);
  if (error_code == 0) console.log("Successful !");
  else console.log("Error-code:", error_code);

  console.log('Getting size for file "/zero/one/INDEX.html" ...');
  let file_size = ctypes.size_t(0);
  error_code = c_get_file_size("/zero/one/INDEX.html", file_size.address());
  if (error_code == 0) {
    console.log("File size in bytes:", file_size.value);
    console.log("Successful !");
  } else console.log("Error-code:", error_code);

  console.log('Getting contents of file "/zero/one/INDEX.html" ...');
  let Uint8Array_t = ctypes.ArrayType(ctypes.uint8_t, file_size.value);
  let file_content = Uint8Array_t();
  error_code = c_get_file_content("/zero/one/INDEX.html", file_content.addressOfElement(0));
  if (error_code == 0) {
    console.log("File content:", file_content.readString());
    console.log("Successful !");
  } else console.log("Error-code:", error_code);
  console.log("=========== Test End =============")

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
