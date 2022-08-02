// async function init() {
//   const wasm = await WebAssembly.instantiateStreaming(fetch("./test.wasm"));
//   debugger;
//   const { greet } = wasm.instance.exports;
//   document.write(greet("jxlust!"));
// }
// init();
import { greet } from "snake_game";

document.write(greet("jxlust!"));
