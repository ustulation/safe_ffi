var abc = require('./build/Release/nodejs_sample')

console.log("Creating path \"/zero/\" ...")
var error_code = abc.c_create_sub_directory("/zero", false)
if (error_code != 0) {
    throw "Unsucessful .. Error: " + error_code
}
console.log("Successful !")

console.log("Creating path \"/zero/one\" ...")
error_code = abc.c_create_sub_directory("/zero/one/", false)
if (error_code != 0) {
    throw "Unsucessful .. Error: " + error_code
}
console.log("Successful !")

console.log("Creating path \"/zero/one/SomeFile.log\" ...")
error_code = abc.c_create_file("/zero/one/SomeFile.log", "This is the content of the file.")
if (error_code != 0) {
    throw "Unsucessful .. Error: " + error_code
}
console.log("Successful !")

console.log("Exiting ...")
