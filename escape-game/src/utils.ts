/**
 * 射线法
 * @param  point  { x: number; y: number }
 * @param polylinePoints  { x: number; y: number }[]
 */
export function checkPointInArea(
  point: { x: number; y: number },
  polylinePoints: any[]
) {
  let leftPoint = 0;
  const A = point;

  for (let i = 0, len = polylinePoints.length; i < len; ++i) {
    //取每一根线来跟A点判断
    let B, C;
    //结尾点和开头点组合
    let { x, y } = polylinePoints[i];
    B = {
      x,
      y,
    };
    let nextIndex = i === len - 1 ? 0 : i + 1;
    C = {
      x: polylinePoints[nextIndex].x,
      y: polylinePoints[nextIndex].y,
    };

    //射线法判断A，B-C左侧是否相交

    //1. B and C maxY < A.y or minY > A.y ,impossible to intersect
    //2. check x is only left
    //3. B.x and C.x > A.x is not in left
    let bcArray = [B.y, C.y];
    bcArray.sort((a, b) => a - b);
    //max > y && min < y can intersect
    if (bcArray[1] > A.y && bcArray[0] < A.y) {
      //? 这里是直接取线段完全处于左侧还是可以横跨左右？
      //也就是取&& 还是 ||
      if (B.x < A.x && C.x < A.x) {
        leftPoint++;
      }
    }
  }

  return leftPoint % 2 === 1;
}

function testCheck() {
  let point = { x: 5, y: 5 };
  let polylinePoints = [
    { x: 0, y: 0 },
    { x: 10, y: 0 },
    { x: 10, y: 10 },
    { x: 0, y: 10 },
  ];
  console.log("checkPointInArea", checkPointInArea(point, polylinePoints));
}
