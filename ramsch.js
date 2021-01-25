const arrow_wasm = require("./pkg");
const fs = require("fs");
const path = require("path");

const filePath = path.join(__dirname, "./flights-10k.arrow");
const file = fs.readFileSync(filePath);

console.log(arrow_wasm.test(file).toJSON());
