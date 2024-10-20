const programBytes = await Bun.file("program.wasm").arrayBuffer();

const { instance } = await WebAssembly.instantiate(programBytes, {
  env: {
    print_int: (int) => console.log("[WASM print_int]", int),
  }
})

const result = instance.exports.add_two_int_32(1, 2);
console.log(result);
