import init, { World } from 'snake_game';

function start() {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const WORLD_SIZE = Math.pow(WORLD_WIDTH,2);
  const SNAKE_SPAWN_IDX = /* Date.now() % WORLD_SIZE */ 0;
  
  const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);
  
  /** @type {HTMLCanvasElement} */
  const canvas = document.getElementById('snake-canvas');
  const ctx = canvas.getContext('2d');
  canvas.height = WORLD_WIDTH * CELL_SIZE;
  canvas.width = WORLD_WIDTH * CELL_SIZE;

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
    const snakeIdx = world.snake_head_idx();
    const col = snakeIdx % WORLD_WIDTH;
    const row = Math.floor(snakeIdx / WORLD_WIDTH);
    ctx.beginPath();
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function paint() {
    drawWorld();
    drawSnake();
  }

  function update() {
    const fps = 10;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.update();
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
