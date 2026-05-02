import init, { Direction, GameStatus, World } from 'snake_game';
import { rnd } from './util/rnd.js';

/** @param {import('snake_game').InitOutput} wasm  */
function start(wasm) {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 4;
  const WORLD_SIZE = Math.pow(WORLD_WIDTH,2);
  const SNAKE_HEAD_IDX = rnd(WORLD_SIZE);
  
  const world = World.new(WORLD_WIDTH, SNAKE_HEAD_IDX);
  
  /** @type {HTMLCanvasElement} */
  const canvas = document.getElementById('snake-canvas');
  const ctx = canvas.getContext('2d');
  canvas.height = WORLD_WIDTH * CELL_SIZE;
  canvas.width = WORLD_WIDTH * CELL_SIZE;

  document.getElementById('game-ctrl-btn').addEventListener('click', () => {
    const status = world.status();
    if (void 0 == status) {
      world.start();
      update();
      document.getElementById('game-ctrl-btn').textContent = '刷新';
    } else {
      location.reload();
    }
  });

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

  function drawReward() {
    const idx = world.reward_idx();
    if (void 0 == idx) {
      return;
    }
    const col = idx % WORLD_WIDTH;
    const row = Math.floor(idx / WORLD_WIDTH);
    ctx.save();
    ctx.fillStyle = '#f00';
    ctx.fillRect(
      col * CELL_SIZE,
      row * CELL_SIZE,
      CELL_SIZE,
      CELL_SIZE
    );
    ctx.restore();
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

  function drawGameStatus() {
    document.querySelector('label + span').textContent = world.status_txt();;
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  }

  function update() {
    if ([GameStatus.Lost, GameStatus.Won].includes(world.status())) { return; }

    const fps = 2;
    setTimeout(() => {
      console.log('playing')
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  paint();
  drawGameStatus();
}

function main() {
  init()
  .then(start);
}

main();
