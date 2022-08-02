import init, { World } from "snake_game";

/** @type {HTMLCanvasElement} */
init().then((_) => {
  const world = World.new();
  const CELL_SIZE = 20; //cell size 10
  const worldWidth = world.width();

  const canvas = document.getElementById("snake-canvas");
  canvas.width = CELL_SIZE * worldWidth;
  canvas.height = CELL_SIZE * worldWidth;

  const ctx = canvas.getContext("2d");
  function drawWorld() {
    ctx.beginPath();
    ctx.strokeStyle = "blue";
    ctx.strokeWidth = "1px";

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(x * CELL_SIZE, 0);
      ctx.lineTo(x * CELL_SIZE, CELL_SIZE * worldWidth);
    }
    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, y * CELL_SIZE);
      ctx.lineTo(CELL_SIZE * worldWidth, CELL_SIZE * y);
    }
    ctx.stroke();
  }
  function drawSnake() {
    const snakeIndex = world.snake_header();
    const row = (snakeIndex / worldWidth) | 0;
    const col = snakeIndex % worldWidth;
    console.log(row, col);

    ctx.beginPath();
    // ctx.fillStyle = "yellow";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }
  // debugger;
  drawWorld();
  drawSnake();
});
