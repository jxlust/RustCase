import init, { World, Direction, GameStatus } from "snake_game";
import { rng } from "./utils/rng";
init().then((wasm) => {
  const INTERVAL_TIME = 1000 / 2;
  const WORLD_WIDTH = 3;
  const CELL_SIZE = 20; //cell size 10

  // const snakeSpawnIndex = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);
  const snakeSpawnIndex = rng(WORLD_WIDTH * WORLD_WIDTH);
  const world = World.new(WORLD_WIDTH, snakeSpawnIndex);

  const worldWidth = world.width();

  const playIdDom = document.getElementById("playId");
  const statusIdDom = document.getElementById("statusId");
  const pointsIdDom = document.getElementById("pointsId");

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

  playIdDom.addEventListener("click", (_) => {
    let gameStatus = world.game_status();
    if (gameStatus === undefined) {
      world.start_game();
      loopStart();

      playIdDom.innerText = "Playing";
    } else {
      window.location.reload();
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
    // debugger;
    // const snakeIndex = world.snake_header();

    //way1: filter
    //way2: slice + reverse
    snakeCells
      // .filter((idx, i) => !(i > 0 && idx === snakeCells[0]))
      .slice()
      .reverse() //i === snakeCells.length - 1
      .forEach((cellIdx, i) => {
        let color = i === snakeCells.length - 1 ? "#a8f8f8" : "#000000";
        ctxFillCell(cellIdx, color);
      });

    //way3: fill header again
    // let headerIndex = snakeCells[0];
    // ctxFillCell(headerIndex, "#a8f8f8");

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
    if (foodIndex) {
      ctxFillCell(foodIndex, "#ff0000");
    }
  }

  function drawStatusText() {
    if (
      world.game_status() === GameStatus.Won ||
      world.game_status() === GameStatus.Lost
    ) {
      playIdDom.textContent = "Re-Play";
    }
    statusIdDom.textContent = world.game_status_text();
    pointsIdDom.textContent = world.points().toString();
  }

  // debugger;
  function updatedView() {
    console.log("111upupda vie");
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawWorld();
    drawSnake();
    drawFoodCell();
    drawStatusText();
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

  updatedView();
});
