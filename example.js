const lib = require("./pkg/arrow_wasm");
const fs = require('fs');
const path = require("path");

const filePath = path.join(__dirname, "./flights-10k.arrow");
const file = fs.readFileSync(filePath);

console.log(lib.parse(file));
