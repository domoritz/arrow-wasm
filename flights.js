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
console.log(batch.column(3).as_f32_vector().toArray());

function sum() {
  const vec = batch.column(3).as_f32_vector();
  console.time("sum wasm get");
  let sum = 0;
  const l = vec.length;
  for (let i = 0; i < l; i++) {
    sum += vec.get(i);
  }
  console.log(sum);
  console.timeEnd("sum wasm get");

  console.time("sum wasm kernel");
  console.log(batch.column(3).as_f32_vector().sum());
  console.timeEnd("sum wasm kernel");

  const arr = batch.column(3).as_f32_vector().toArray();
  console.time("sum native js");
  console.log(arr.reduce((a, b) => a + b, 0));
  console.timeEnd("sum native js");
}

sum();
