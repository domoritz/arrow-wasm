const arrow_wasm = require("../pkg/arrow_wasm");
const fs = require("fs");
const path = require("path");

const filePath = path.join(__dirname, "./chicago-crimes-2018.arrow");
const file = fs.readFileSync(filePath);

const table = arrow_wasm.Table.from(file);

console.log(table.numBatches);
// console.log(table.schema.toJSON().fields);

for (let i = 0; i < 5; i++) {
  const strCol = table.recordBatch(0).columnWithName("PrimaryType");

  console.time("toJSON");
  strCol.asStringVector().toJSON();
  console.timeEnd("toJSON");
}

console.log();

for (let i = 0; i < 5; i++) {
  const numberCol = table.recordBatch(0).columnWithName("Latitude");

  console.time("toJSON");
  numberCol.asFloat32Vector().toJSON();
  console.timeEnd("toJSON");

  console.time("toArray");
  numberCol.asFloat32Vector().toArray();
  console.timeEnd("toArray");

  console.time("view");
  numberCol.asFloat32Vector().view();
  console.timeEnd("view");
}
