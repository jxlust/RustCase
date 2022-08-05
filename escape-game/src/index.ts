enum FillStatus {
  None = 0,
  Fill = 1,
}
class CircleCell {
  public X: number;
  public Y: number;
  private isFilled: number;

  constructor(x: number, y: number) {
    this.X = x;
    this.Y = y;
    this.isFilled = FillStatus.None;
  }

  getIsFilled() {
    return this.isFilled;
  }

  setIsFilled() {
    //fill
    this.isFilled = FillStatus.Fill;
  }
}

const WIDTH = 11; //every row or col cell count
const CELL_SIZE = WIDTH * WIDTH;
const CIRCLE_RADIUS = 15;

const START_X = 30;
const START_Y = 30;
const CELL_SPACE_X = 4;
const CELL_SPACE_Y = 6;
const RANDOM_COUNT = 3;
const FILL_COLOR = "red";
const CELL_COLOR = "orange";

//save all circle center pointer
const circleList: CircleCell[] = [];
//init random fill cell
let randomList: number[] = [];

//animal position pointer
//todo: idx may appear on the edge,need fix it
let animalIdx: number;
let ganmeStaus = 1; //0:end  1: playing

const paddingRow = START_X - CIRCLE_RADIUS;
const paddingCol = START_Y - CIRCLE_RADIUS;

const resultDom = document.getElementById("resultId");
const stepIdDom = document.getElementById("stepId");
const restartIdDom = document.getElementById("restartId");
let step = 0;

let animalElement: any;

const myCanvas = <HTMLCanvasElement>document.getElementById("myCanvas");
const ctx = myCanvas.getContext("2d");

const canvasWidth =
  paddingRow * 2 +
  WIDTH * 2 * CIRCLE_RADIUS +
  (WIDTH - 1) * CELL_SPACE_X +
  CIRCLE_RADIUS +
  CELL_SPACE_X / 2;

const canvasHeight =
  paddingCol * 2 + WIDTH * 2 * CIRCLE_RADIUS + (WIDTH - 1) * CELL_SPACE_Y;
myCanvas.width = canvasWidth;

myCanvas.height = canvasHeight;

function drawWorld() {
  // for(let x = 0 ; x < )
  ctx.fillStyle = "#000000";
  ctx.globalAlpha = 0.2;
  ctx.fillRect(0, 0, myCanvas.width, myCanvas.height);
}
function rng(n: number) {
  return (Math.random() * n) | 0;
}

function randomFillCell() {
  while (true) {
    let rIndex = rng(CELL_SIZE);
    if (!randomList.includes(rIndex) && rIndex !== animalIdx) {
      randomList.push(rIndex);
    }
    if (randomList.length === RANDOM_COUNT) {
      break;
    }
  }
}

async function getAnimalData() {
  return new Promise<any>((resolve) => {
    let animalElement = new Image();
    animalElement.src = "./img/user.png";
    animalElement.onload = () => {
      resolve(animalElement);
    };
    animalElement.onerror = () => {
      resolve(null);
    };
  });
}
function generateCell() {}

function drawCell() {
  ctx.globalAlpha = 1;

  for (let x = 0; x < WIDTH; x++) {
    for (let y = 0; y < WIDTH; y++) {
      let pX = START_X + y * (2 * CIRCLE_RADIUS + CELL_SPACE_X);
      let pY = START_Y + x * (2 * CIRCLE_RADIUS + CELL_SPACE_Y);
      if (x % 2 === 1) {
        pX += CIRCLE_RADIUS + CELL_SPACE_X / 2;
      }
      let pIndex = x * WIDTH + y;
      let cell = new CircleCell(pX, pY);
      let fillColor = CELL_COLOR;
      if (randomList.includes(pIndex)) {
        cell.setIsFilled();
        fillColor = FILL_COLOR;
      }
      circleList.push(cell);

      ctx.fillStyle = fillColor;
      ctx.beginPath();
      ctx.arc(pX, pY, CIRCLE_RADIUS, 0, 2 * Math.PI);
      ctx.fill();
    }
  }
}

function fillLastAnimal(lastIdx: number) {
  let circleCell = getPoint(lastIdx);
  let [pX, pY] = [circleCell.X, circleCell.Y];
  ctx.fillStyle = CELL_COLOR;
  ctx.beginPath();
  ctx.arc(pX, pY, CIRCLE_RADIUS, 0, 2 * Math.PI);
  ctx.fill();
}
async function drawAnimal(nextId?: number) {
  if (nextId !== undefined) {
    //fill last
    fillLastAnimal(animalIdx);
    animalIdx = nextId;
  }
  let imgWidth = 20;
  let imgOffset = 20 / 4;

  let circleCell = getPoint(animalIdx);
  let [pX, pY] = [circleCell.X, circleCell.Y];
  ctx.beginPath();
  ctx.drawImage(
    animalElement,
    pX - CIRCLE_RADIUS + imgOffset,
    pY - CIRCLE_RADIUS + imgOffset,
    imgWidth,
    imgWidth
  );
}

function getPoint(idx: number) {
  if (idx >= 0 && idx < circleList.length) {
    return circleList[idx];
  }
  return null;
}
/**
 *
 * @param point p
 * @param circle  circle center
 * @param r radius
 */
function checkIsInCircle(point: number[], circle: number[], r: number) {
  let xDiff = point[0] - circle[0];
  let yDiff = point[1] - circle[1];
  return xDiff * xDiff + yDiff * yDiff <= r * r;
}

function getClickIdx(offsetX: number, offsetY: number, lists: CircleCell[]) {
  let idx = -1;
  for (let i = 0, len = lists.length; i < len; ++i) {
    let cell = lists[i];
    let [pX, pY] = [cell.X, cell.Y];
    let isInCircle = checkIsInCircle(
      [offsetX, offsetY],
      [pX, pY],
      CIRCLE_RADIUS
    );
    if (isInCircle) {
      idx = i;
      break;
    }
  }
  return idx;
}
function updateCellColor(idx: number) {
  let circleCell = getPoint(idx);
  if (circleCell.getIsFilled() === FillStatus.None) {
    //not fill,to do fill this cell
    let [pX, pY] = [circleCell.X, circleCell.Y];
    circleCell.setIsFilled();

    ctx.fillStyle = FILL_COLOR;
    ctx.beginPath();
    ctx.arc(pX, pY, CIRCLE_RADIUS, 0, 2 * Math.PI);
    ctx.fill();
  }
}
function updateStepView() {
  stepIdDom.innerText = `${step}`;
}

function checkIsOnWallEdge(idx: number) {
  let y = (idx / WIDTH) | 0;

  let x = idx % WIDTH;
  //is in the wall edge 边缘
  if (x === 0 || y === 0 || x === WIDTH - 1 || y === WIDTH - 1) {
    return true;
  }
  return false;
}
//animal 6 dir can go
function animalStep() {
  //check is can skip
  let isCanSkip = checkCanSkip(animalIdx);
  if (isCanSkip) {
    // change animalIdx
    let nbs = getNeighbors(animalIdx);
    let nextId = nbs[rng(nbs.length)];
    drawAnimal(nextId);
    //check animal is in the wall edge 边缘
    if (checkIsOnWallEdge(nextId)) {
      resultDom.innerText = "被你逃走了,\u{1F33A}";
      ganmeStaus = 0;
    }
  } else {
    //catch  it
    resultDom.innerText = "抓住你了，哈哈哈！\u{1F606}";
    ganmeStaus = 0;
  }
}

function getNeighbors(idx: number) {
  // (x,y) => 6 kinds dir
  // (x-1,y) (x+1,y)

  //方向这里还需要根据奇偶性判断
  // y 奇数行
  //(x,y-1) (x+1,y-1)
  //(x,y+1) (x+1,y+1)
  // y 偶数行
  //(x-1,y-1) (x,y-1)
  //(x-1,y+1) (x,y+1)

  let y = (idx / WIDTH) | 0;
  let x = idx % WIDTH;
  let neighbors: number[] = [];
  if (x - 1 >= 0) {
    //left
    if (circleList[idx - 1].getIsFilled() === FillStatus.None) {
      neighbors.push(idx - 1);
    }
  }
  if (x + 1 < WIDTH) {
    //right
    if (circleList[idx + 1].getIsFilled() === FillStatus.None) {
      neighbors.push(idx + 1);
    }
  }
  let colOffset1 = 0;
  let colOffset2 = 0;
  if (y % 2 === 1) {
    //奇数行
    colOffset1 = 0;
    colOffset2 = +1;
  } else {
    colOffset1 = -1;
    colOffset2 = 0;
  }
  if (y - 1 >= 0 && x + colOffset1 >= 0) {
    // top
    let pos = (y - 1) * WIDTH + x + colOffset1;
    if (circleList[pos].getIsFilled() === FillStatus.None) {
      neighbors.push(pos);
    }
  }
  if (y - 1 >= 0 && x + colOffset2 < WIDTH) {
    //right top
    let pos = (y - 1) * WIDTH + x + colOffset2;
    if (circleList[pos].getIsFilled() === FillStatus.None) {
      neighbors.push(pos);
    }
  }
  if (y + 1 < WIDTH && x + colOffset1 >= 0) {
    // bottom
    let pos = (y + 1) * WIDTH + x + colOffset1;
    if (circleList[pos].getIsFilled() === FillStatus.None) {
      neighbors.push(pos);
    }
  }
  if (y + 1 < WIDTH && x + colOffset2 < WIDTH) {
    //right bottom
    let pos = (y + 1) * WIDTH + x + colOffset2;
    if (circleList[pos].getIsFilled() === FillStatus.None) {
      neighbors.push(pos);
    }
  }
  return neighbors;
}

function checkCanSkip(curIdx: number) {
  //dfs stack
  let stack: number[] = [curIdx];
  let visited = new Set();
  visited.add(curIdx);
  while (stack.length) {
    let cur = stack.pop();
    let y = (cur / WIDTH) | 0;
    let x = cur % WIDTH;
    //is in the wall edge 边缘
    if (x === 0 || y === 0 || x === WIDTH - 1 || y === WIDTH - 1) {
      return true;
    }
    let neighbors: number[] = getNeighbors(cur);
    for (let nb of neighbors) {
      if (!visited.has(nb)) {
        stack.push(nb);
        visited.add(nb);
      }
    }
  }

  return false;
}

myCanvas.addEventListener("click", (e) => {
  let { offsetX, offsetY } = e;
  let idx = getClickIdx(offsetX, offsetY, circleList);
  console.log("idx:", idx);
  if (idx >= 0 && idx !== animalIdx && ganmeStaus) {
    //check is can skip
    // debugger;
    step++;
    updateCellColor(idx);
    updateStepView();
    animalStep();
  }
});
function randomFirstIdx() {
  while (true) {
    let idx = rng(CELL_SIZE);
    if (!checkIsOnWallEdge(idx)) {
      animalIdx = idx;
      break;
    }
  }
}
async function init() {
  randomFirstIdx();

  animalElement = await getAnimalData();
  randomFillCell();
  drawWorld();
  drawCell();
  drawAnimal();
}

init();

restartIdDom.addEventListener("click", () => {
  window.location.reload();
});
