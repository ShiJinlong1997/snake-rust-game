import * as snake from 'snake_game';
console.log(snake)

function main() {
  const memory = new WebAssembly.Memory({ initial: 1 });

  const importObject = {
    js: {
      mem: memory,
    },
    console: {
      log: () => {
        console.log('log');
      },
      error: () => {
        console.error('error');
      },
    },
  };

  fetch('/import_js_mem.wasm')
  .then(response => response.arrayBuffer())
  .then(buffer => WebAssembly.instantiate(buffer, importObject))
  .then(wasm => {
    const wasmMod = wasm.instance.exports;
    console.log(wasmMod.sum(10, 20));
    
    // 不再用 wasm 导出的内存，用 WebAssembly.Memory 实例
    // - const uint8Array = new Uint8Array(wasmMod.mem.buffer, 0, 2);
    // +
    const uint8Array = new Uint8Array(memory.buffer, 0, 2);
    const txt = new TextDecoder().decode(uint8Array)
    console.log(txt)
  });
}

main();
