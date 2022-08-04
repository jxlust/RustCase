const WIDTH = 11; //每行cell个数
const CELL_SIZE = 30;
const CIRCLE_RADIUS = 15;

const START_X = 30;
const START_Y = 30;
const CELL_SPACE = 10;

const paddingRow = START_X - CIRCLE_RADIUS;
const paddingCol = START_Y - CIRCLE_RADIUS;

const myCanvas = <HTMLCanvasElement>document.getElementById("myCanvas");
const ctx = myCanvas.getContext("2d");

const canvasWidth =
  paddingRow * 2 + WIDTH * 2 * CIRCLE_RADIUS + (WIDTH - 1) * CELL_SPACE;

const canvasHeight =
  paddingCol * 2 + WIDTH * 2 * CIRCLE_RADIUS + (WIDTH - 1) * CELL_SPACE;
myCanvas.width = canvasWidth;
myCanvas.height = canvasWidth;

function drawWorld() {
  // for(let x = 0 ; x < )
  ctx.fillStyle = "#000000";
  ctx.globalAlpha = 0.25;
  ctx.fillRect(0, 0, myCanvas.width, myCanvas.height);
}

async function getAnimalData() {
  return new Promise<any>((resolve) => {
    let animalElement = new Image();
    animalElement.src =
      "https://img0.baidu.com/it/u=3704427174,3815146258&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=500";
    animalElement.onload = () => {
      console.log(11111, 22);
      resolve(animalElement);
    };
    animalElement.onerror = () => {
      resolve(null);
    };
  });
}
function drawCell() {
  ctx.beginPath();
  ctx.globalAlpha = 1;
  ctx.fillStyle = "orange";

  for (let x = 0; x < WIDTH; x++) {
    for (let y = 0; y < WIDTH; y++) {
      let pX = START_X + y * (2 * CIRCLE_RADIUS + CELL_SPACE);
      let pY = START_Y + x * (2 * CIRCLE_RADIUS + CELL_SPACE);
      ctx.beginPath();
      ctx.arc(pX, pY, CIRCLE_RADIUS, 0, 2 * Math.PI);
      ctx.fill();
    }
  }
}

async function drawAnimal() {
  let img = await getAnimalData();

  let pointIdx = 5;

  let row = (pointIdx / WIDTH) | 0;
  let col = pointIdx % WIDTH;
  let { pX, pY } = getPoint(row, col);
  ctx.beginPath();
  ctx.drawImage(img, pX - CIRCLE_RADIUS + 5, pY - CIRCLE_RADIUS + 5, 20, 20);
}

function getPoint(row: number, col: number) {
  let pX = START_X + col * (2 * CIRCLE_RADIUS + CELL_SPACE);
  let pY = START_Y + row * (2 * CIRCLE_RADIUS + CELL_SPACE);
  return {
    pX,
    pY,
  };
}

drawWorld();
drawCell();
drawAnimal();
