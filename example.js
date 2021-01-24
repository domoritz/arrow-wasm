const arrow_wasm = require("./pkg");
const arrow_js = require("apache-arrow");
const { ListBuilder } = require("apache-arrow");


function rint(min, max) {
  let delta = min;
  if (max === undefined) {
    min = 0;
  } else {
    delta = max - min;
  }
  return (min + delta * Math.random()) | 0;
}

function ints(n, min, max, nullf) {
  const data = [];
  for (let i = 0; i < n; ++i) {
    const v = nullf && Math.random() < nullf ? null : rint(min, max);
    data.push(v);
  }
  return data;
}

// const N = 1e7;
const N = 1e3

const values = ints(N, -1000, 1000);

const vector_js = arrow_js.Int32Vector.from(values);
const vector_wasm = arrow_wasm.Int32Vector.from(values);

function run(a, wasm) {
  // console.log('Array', a)

  console.log("length", a.length);
  console.log("get", a.get(0));
  console.log("isValid", a.isValid(0));

  console.time("scan");
  let sum = 0;
  for (let index = 0; index < N; index++) {
    sum += a.get(index);
  }
  console.timeEnd("scan");
  console.log("sum", sum);

  console.log("toArray", a.toArray());
  console.log("toJSON", a.toJSON());
}

console.group("==== JS ====");
run(vector_js, false);
console.groupEnd();

console.log();

console.group("==== WASM ====");
run(vector_wasm, true);
console.groupEnd();
