const arrow_wasm = require("../pkg/arrow_wasm");
const fs = require("fs");
const path = require("path");

const filePath = path.join(__dirname, "./flights-10k.arrow");
const file = fs.readFileSync(filePath);

const table = arrow_wasm.Table.from(file);

console.log(table.numBatches());
console.log(table.schema());

const batch = table.recordBatch(0);

console.log(batch.numRows());
console.log(batch.numColumns());
console.log(batch.schema().toJSON());
console.log(batch.schema().fields());
console.log(batch.schema().field(0).dataType().toJSON())
console.log(batch.schema().columnWithName("DEP_DELAY"));
console.log(batch.column(3).toString());
console.log(batch.column(3).asFloat32Vector().toArray());

function sum() {
  const vec = batch.column(3).asFloat32Vector();
  console.time("sum wasm get");
  let sum = 0;
  const l = vec.length;
  for (let i = 0; i < l; i++) {
    sum += vec.get(i);
  }
  console.log(sum);
  console.timeEnd("sum wasm get");

  console.time("sum wasm kernel");
  console.log(batch.column(3).asFloat32Vector().sum());
  console.timeEnd("sum wasm kernel");

  console.time("sum native js");
  const arr = batch.column(3).asFloat32Vector().toArray();
  console.log(arr.reduce((a, b) => a + b, 0));
  console.timeEnd("sum native js");

  console.time("sum native js view");
  const view = batch.column(3).asFloat32Vector().view();
  console.log(view.reduce((a, b) => a + b, 0));
  console.timeEnd("sum native js view");
}

sum();
