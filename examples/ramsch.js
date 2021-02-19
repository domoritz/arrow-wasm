const arrow_wasm = require("../pkg/arrow_wasm");
const arrow_js = require("apache-arrow");

function rint(min, max) {
  let delta = min;
  if (max === undefined) {
    min = 0;
  } else {
    delta = max - min;
  }
  return (min + delta * Math.random()) | 0;
}

function ints(n, min, max, nullf, typed) {
  const data = [];
  for (let i = 0; i < n; ++i) {
    const v = nullf && Math.random() < nullf ? null : rint(min, max);
    data.push(v);
  }
  if (typed) {
    return new Int32Array(data);
  }
  return data;
}

// const N = 1e7;
const N = 1e3;

const values = ints(N, -1000, 1000, false);

const vector_js = arrow_js.Int32Vector.from(values);
const vector_wasm = arrow_wasm.Int32Vector.from(values);

function run(a) {
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
  if (N < 1e4) {
    console.log("toJSON", a.toJSON());
  }

  console.log("toString", a.toString());
}

// try different types

const vectors = [];
vectors.push(arrow_wasm.Int8Vector.from([1, 2, 3]));
vectors.push(arrow_wasm.Int16Vector.from([1, 2, 3]));
vectors.push(arrow_wasm.Int32Vector.from([1, 2, 3]));
vectors.push(arrow_wasm.Int64Vector.from([1, 2, 3].map(BigInt)));

vectors.push(arrow_wasm.Uint8Vector.from([1, 2, 3]));
vectors.push(arrow_wasm.Uint16Vector.from([1, 2, 3]));
vectors.push(arrow_wasm.Uint32Vector.from([1, 2, 3]));
vectors.push(arrow_wasm.Uint64Vector.from([1, 2, 3].map(BigInt)));

vectors.push(arrow_wasm.Float32Vector.from([1.1, 2.2, 3.3]));
vectors.push(arrow_wasm.Float64Vector.from([1.1, 2.2, 3.3]));

vectors.push(arrow_wasm.BooleanVector.from([5], 4));

console.log(vectors.map((v) => v.toJSON()));

// comparisons

console.group("==== Native ====");

console.time("scan");
let sum = 0;
for (let index = 0; index < N; index++) {
  sum += values[index];
}
console.timeEnd("scan");
console.log("sum", sum);
console.groupEnd();

console.log();

console.group("==== WASM ====");
run(vector_wasm);
console.groupEnd();

console.log();

console.group("==== JS ====");
run(vector_js);
console.groupEnd();

const raw = arrow_wasm.Int32Vector.from([1, 2, 3]).toRaw();
console.log(raw);
console.log(
  arrow_wasm.Vector.fromRaw(raw.array, raw.schema).asInt32Vector().length
);
