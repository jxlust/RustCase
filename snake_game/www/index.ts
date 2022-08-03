import init, { World, Direction } from "snake_game";

init().then((wasm) => {
  const INTERVAL_TIME = 1000 / 4;
  const WORLD_WIDTH = 16;
  const CELL_SIZE = 20; //cell size 10

  const snakeSpawnIndex = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);
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

    snakeCells.forEach((cellIdx, i) => {
      // const snakeIndex = world.snake_header();
      const row = (cellIdx / worldWidth) | 0;
      const col = cellIdx % worldWidth;
      console.log(row, col);
      ctx.beginPath();
      ctx.fillStyle = i === 0 ? "#a8f8f8" : "#000000";
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      ctx.stroke();
    });

    world.step();
  }

  // debugger;
  function updatedView() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawWorld();
    drawSnake();
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
