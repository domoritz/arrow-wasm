const arrow_wasm = require("../pkg/arrow_wasm");

const schema = arrow_wasm.Schema.from({
  fields: [
    {
      name: "number",
      nullable: false,
      type: {
        name: "int",
        isSigned: true,
        bitWidth: 32,
      },
    },
  ],
});

console.log(schema.toJSON());
