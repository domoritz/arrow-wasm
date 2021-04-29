const b = require("benny");
const arrow_wasm = require("./pkg/arrow_wasm");
const arrow_js = require("apache-arrow");
const fs = require("fs");
const path = require("path");

const filePath = path.join(__dirname, "./examples/flights-1m.arrow");
const file = fs.readFileSync(filePath);

const t_wasm = arrow_wasm.Table.from(file);
const t_js = arrow_js.Table.from(file);

const batch = t_wasm.recordBatch(0);

b.suite(
  "Load",

  b.add("Arrow WASM", () => {
    arrow_wasm.Table.from(file);
  }),

  b.add("Arrow JS", () => {
    arrow_js.Table.from(file);
  }),

  b.cycle(),
  b.complete()
);

b.suite(
  "Serialize",

  b.add("Arrow WASM", () => {
    t_wasm.serialize();
  }),

  b.add("Arrow JS", () => {
    t_js.serialize();
  }),

  b.cycle(),
  b.complete()
);

b.suite(
  "Schema",

  b.add("Arrow WASM", () => {
    t_wasm.schema;
  }),

  b.add("Arrow JS", () => {
    t_js.schema;
  }),

  b.cycle(),
  b.complete()
);

b.suite(
  "To Array",

  b.add("Arrow WASM", () => {
    batch.column(3).asFloat32Vector().toArray();
  }),

  b.add("Arrow JS", () => {
    t_js.getColumnAt(3).toArray();
  }),

  b.cycle(),
  b.complete()
);

b.suite(
  "Sum",

  b.add("Arrow WASM", () => {
    const vec = batch.column(3).asFloat32Vector();
    let sum = 0;
    for (let i = 0; i < vec.length; i++) {
      sum += vec.get(i);
    }
  }),

  b.add("Arrow WASM (kernel)", () => {
    const vec = batch.column(3).asFloat32Vector();
    let sum = vec.sum();
  }),

  b.add("Arrow JS", () => {
    const vec = t_js.getColumnAt(3);
    let sum = 0;
    for (let i = 0; i < vec.length; i++) {
      sum += vec.get(i);
    }
  }),

  b.cycle(),
  b.complete()
);
