const arrow_wasm = require("../pkg/arrow_wasm");

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

const N = 1e7;
const repeats = 10;

const values = ints(N, -1000, 1000, false);

const vec = arrow_wasm.Int32Vector.from(values);

function sum() {
  console.time("sum wasm get");
  let sum = 0;
  const l = vec.length;
  for (let i = 0; i < l; i++) {
    sum += vec.get(i);
  }
  console.log(sum);
  console.timeEnd("sum wasm get");

  console.time("sum wasm kernel");
  for (let index = 0; index < repeats; index++) console.log(vec.sum());
  console.timeEnd("sum wasm kernel");

  console.time("sum native js (toJSON)");
  const narr = vec.toJSON();
  for (let index = 0; index < repeats; index++)
    console.log(narr.reduce((a, b) => a + b, 0));
  console.timeEnd("sum native js (toJSON)");

  console.time("sum native js (toArray)");
  const arr = vec.toArray();
  for (let index = 0; index < repeats; index++)
    console.log(arr.reduce((a, b) => a + b, 0));
  console.timeEnd("sum native js (toArray)");

  console.time("sum native js (view)");
  const view = vec.view();
  for (let index = 0; index < repeats; index++)
    console.log(view.reduce((a, b) => a + b, 0));
  console.timeEnd("sum native js (view)");
}

sum();
console.log();
sum();
console.log();
sum();
