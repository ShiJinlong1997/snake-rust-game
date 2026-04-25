import init, { Direction, World } from 'snake_game';

/** @param {import('snake_game').InitOutput} wasm  */
function start(wasm) {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const WORLD_SIZE = Math.pow(WORLD_WIDTH,2);
  const SNAKE_SPAWN_IDX = 10 /* Date.now() % WORLD_SIZE */;
  
  const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);
  
  /** @type {HTMLCanvasElement} */
  const canvas = document.getElementById('snake-canvas');
  const ctx = canvas.getContext('2d');
  canvas.height = WORLD_WIDTH * CELL_SIZE;
  canvas.width = WORLD_WIDTH * CELL_SIZE;

  document.addEventListener('keydown', event => {
    switch (event.code) {
      case 'ArrowUp':
        world.set_snake_dir(Direction.Up);
        break;
      case 'ArrowRight':
        world.set_snake_dir(Direction.Right);
        break;
      case 'ArrowDown':
        world.set_snake_dir(Direction.Down);
        break;
      case 'ArrowLeft':
        world.set_snake_dir(Direction.Left);
        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();

    // 画竖线
    for (let x = 0; x <= WORLD_WIDTH; x += 1) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, WORLD_WIDTH * CELL_SIZE);
    }

    // 画横线
    for (let y = 0; y <= WORLD_WIDTH; y += 1) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(WORLD_WIDTH * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_len()
    );

    snakeCells.forEach(cellIdx => {
      const col = cellIdx % WORLD_WIDTH;
      const row = Math.floor(cellIdx / WORLD_WIDTH);
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });

    snakeCells.slice(0, 1).forEach(cellIdx => {
      const col = cellIdx % WORLD_WIDTH;
      const row = Math.floor(cellIdx / WORLD_WIDTH);
      ctx.save();
      ctx.fillStyle = '#7878db';
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      ctx.restore();
    });
  }

  function paint() {
    drawWorld();
    drawSnake();
  }

  function update() {
    const fps = 10;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  paint();
  update();
}

async function main() {
  init()
  .then(start);
}

main();
