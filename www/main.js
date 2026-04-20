import init, { greet } from 'snake_game';

async function start() {
  const wasm = await init();
  console.log(wasm.greet);
  console.log(greet);
  // debugger
  greet('Loong');
  console.log('OK');
}

start();
