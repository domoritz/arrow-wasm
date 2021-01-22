const lib = require("./pkg/arrow_wasm");
const fs = require('fs');
const path = require("path");

const url =
  "https://gist.githubusercontent.com/domoritz/0f53a5abde95564c36dfaac623a7a922/raw/cce3719b853e25d5dfff97a270283ba83af3c0e6/flights-10k.arrow";

const filePath = path.join(__dirname, "./flights-10k.arrow");
const file = fs.readFileSync(filePath);

// console.log(file)

// console.log(lib.length())

// const uint8arr = new Uint8Array(
//     file.buffer,
//     file.byteOffset,
//     file.length / Uint8Array.BYTES_PER_ELEMENT);

//     console.log(lib.parse(uint8arr));

console.log(lib.parse(file));
