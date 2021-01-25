const arrow_wasm = require("./pkg");
const fs = require("fs");
const path = require("path");

const filePath = path.join(__dirname, "./flights-10k.arrow");
const file = fs.readFileSync(filePath);

const batch = arrow_wasm.test(file);

console.log(batch.numRows());
console.log(batch.numColumns());
console.log(batch.schema().toJSON());
console.log(batch.column(3).toString());
console.log(batch.column(3));
