import init, { World, Direction } from "snake_game";
import { rng } from "./utils/rng";
init().then((wasm) => {
  const INTERVAL_TIME = 1000 / 9;
  const WORLD_WIDTH = 16;
  const CELL_SIZE = 20; //cell size 10

  // const snakeSpawnIndex = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);
  const snakeSpawnIndex = rng(WORLD_WIDTH * WORLD_WIDTH);
  const world = World.new(WORLD_WIDTH, snakeSpawnIndex);

  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  canvas.width = CELL_SIZE * worldWidth;
  canvas.height = CELL_SIZE * worldWidth;
  const ctx = canvas.getContext("2d");

  // debugger;
  document.addEventListener("keydown", (e) => {
    console.log(e.code);
    switch (e.code) {
      case "ArrowUp":
        world.change_snake_dir(Direction.Up);
        break;
      case "ArrowDown":
        world.change_snake_dir(Direction.Down);
        break;
      case "ArrowRight":
        world.change_snake_dir(Direction.Right);
        break;
      case "ArrowLeft":
        world.change_snake_dir(Direction.Left);
        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();
    // ctx.strokeStyle = ["blue","red","yellow"].at(colorIndex);
    ctx.strokeStyle = "skyblue";
    ctx.lineWidth = 1;

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
    const snakeCellsPtr = world.snake_cells();
    const snakeLength = world.snake_length();

    const snakeCells = new Int32Array(
      wasm.memory.buffer,
      snakeCellsPtr,
      snakeLength
    ); //usize = 4 bytes  = 4 * 8

    // const snakeIndex = world.snake_header();
    snakeCells.forEach((cellIdx, i) => {
      let color = i === 0 ? "#a8f8f8" : "#000000";
      ctxFillCell(cellIdx, color);
    });

    //fill header again
    let headerIndex = snakeCells[0];
    ctxFillCell(headerIndex, "#a8f8f8");
    world.step();
  }

  function ctxFillCell(index: number, color?: string) {
    const row = (index / worldWidth) | 0;
    const col = index % worldWidth;
    ctx.beginPath();
    color && (ctx.fillStyle = color);
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function drawFoodCell() {
    let foodIndex = world.food_cell();
    ctxFillCell(foodIndex, "#ff0000");
  }

  // debugger;
  function updatedView() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawWorld();
    drawSnake();
    drawFoodCell();
  }

  function loopStart() {
    let startTime: number;
    function loopCallback(timestamp: number) {
      if (!startTime) {
        startTime = timestamp;
      }
      let diffTime = timestamp - startTime;
      if (diffTime >= INTERVAL_TIME || diffTime === 0) {
        startTime = timestamp;
        updatedView();
      }
      window.requestAnimationFrame(loopCallback);
    }
    window.requestAnimationFrame(loopCallback);
  }

  loopStart();
});
