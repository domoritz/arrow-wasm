const lib = require("./pkg/arrow_wasm");
const fs = require("fs");
const path = require("path");
const {
  Bool,
  Table,
  Vector,
  Uint32,
  Int32,
  Float64,
  Dictionary,
  Utf8,
  RecordBatch,
} = require("apache-arrow");
const { table } = require("arquero");
const { dataFromTable } = require("arquero-arrow");

const filePath = path.join(__dirname, "./flights-10k.arrow");

function parseFlights() {
  const file = fs.readFileSync(filePath);

  console.log(lib.parse(file));
}

// parseFlights()

function serializeFlights() {
  const file = fs.readFileSync(filePath);
  // serialize the arrow table -> returns its input
  lib.serialize(file);
}

// serializeFlights();

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

const values = ints(1e7, -1000, 1000);
// const values = ints(1e1, -1000, 1000);

// batch = lib.serialize_vector(values, true);

// console.log(batch)

/*

// serialize the vector as a batch
batch = lib.serialize_vector(values, true);

// console.log(batch)

const t = RecordBatch.from(batch);
console.log(t.schema);
console.log(t.length);
// console.log(t.toArray());


const dt = table({ values });
const v = dataFromTable(dt, dt.column("values"), new Int32(), true);

// const v = Vector.from({ type: new Int32(), vector, highWaterMark: 1e12 });

const tn = Table.new([v], ["values"]);
const b = tn.serialize();

const t2 = Table.from(b);
console.log(tn.schema);
console.log(tn.length);
// console.log(tn.toArray());

*/

function encode(values, type = new Int32(), nulls = true) {
  const dt = table({ values });

  const u0 = Date.now();
  const u = dataFromTable(dt, dt.column("values"), type, nulls);
  const a = Table.new([u], ["values"]).serialize();
  const ut = Date.now() - u0;

  const v0 = Date.now();
  const v = Vector.from({ type, values, highWaterMark: 1e12 });
  const b = Table.new([v], ["values"]).serialize();
  const vt = Date.now() - v0;

  const w0 = Date.now();
  const c = lib.serialize_vector(values, nulls);
  const wt = Date.now() - w0;

  const j0 = Date.now();
  const js = JSON.stringify(values);
  const jt = Date.now() - j0;
  const j = new TextEncoder().encode(js);

  console.table([
    // eslint-disable-line
    { method: "json", time: jt, bytes: j.length },
    { method: "arquero-arrow", time: ut, bytes: a.length },
    { method: "arrow", time: vt, bytes: b.length },
    { method: "wasm", time: wt, bytes: c.length },
  ]);
}

// encode(values)

function batch() {
  const recordBatch = lib.RecordBatch.from(values);

  console.log("numRows", recordBatch.numRows());
  console.log("numColumns", recordBatch.numColumns());
  console.log("serialize", recordBatch.serialize());
  console.log("schema", recordBatch.schema());

  console.log("nullCount", recordBatch.nullCount());

  // wasm
  const w0 = Date.now();
  let wsum = 0;
  for (let index = 0; index < recordBatch.numRows(); index++) {
    wsum += recordBatch.value(index);
  }
  const wt = Date.now() - w0;

  // wasm implemented
  const wi0 = Date.now();
  let wisum = recordBatch.sum();
  const wit = Date.now() - wi0;

  // arrow
  const vectorArrow = Vector.from({
    type: new Int32(),
    values,
    highWaterMark: 1e12,
  });

  const a0 = Date.now();
  let asum = 0;
  for (let index = 0; index < vectorArrow.length; index++) {
    asum += vectorArrow.get(index);
  }
  const at = Date.now() - a0;

  // native
  const n0 = Date.now();
  let nsum = 0;
  for (let index = 0; index < values.length; index++) {
    nsum += values[index];
  }
  const nt = Date.now() - n0;

  console.table([
    // eslint-disable-line
    { method: "native", time: nt, sum: nsum },
    { method: "arrow", time: at, sum: asum },
    { method: "wasm", time: wt, sum: wsum },
    { method: "wasm implemented", time: wit, sum: wisum },
  ]);
}

batch();
