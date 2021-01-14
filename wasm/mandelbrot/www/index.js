import * as wasm from "hello-wasm-pack";
// wasm.greet();

//　配列を描画する
function draw(ctx, canvas_w, canvas_h, data) {
  let img = new ImageData(new Uint8ClampedArray(data), canvas_w, canvas_h); // new ImageData(array, width [, height]);
  // https://developer.mozilla.org/en-US/docs/Web/API/ImageData/ImageData
  ctx.putImageData(img, 0, 0); // set image on canvas
}

const X_MIN = -1.5;
const X_MAX = 0.5;
const Y_MIN = -1.0;
const Y_MAX = 1.0;
const MAX_ITER = 64;

console.log("start loading wasm");
const mandelbrot = import("../pkg").catch(console.error);

// wasm の読み込みは非同期なので、Promise で読み込み完了まで待って、button の onClick に登録
Promise.all([mandelbrot]).then(async function ([
  { generate_mandelbrot_set, draw_mandelbrot_set },
]) {
  console.log("finished loading");
  const renderBtn = document.getElementById("render");
  renderBtn.addEventListener("click", () => {
    draw_mandelbrot_set();
    let wasmResult = null;
    {
      const CANVAS_ID = "canvas_hybrid";
      let canvas = document.getElementById(CANVAS_ID);
      let context = canvas.getContext("2d");
      const canvasWidth = canvas.width;
      const canvasHeight = canvas.height;

      // time it
      const generateStartTime = Date.now();
      wasmResult = generate_mandelbrot_set(
        canvasWidth,
        canvasHeight,
        X_MIN,
        X_MAX,
        Y_MIN,
        Y_MAX,
        MAX_ITER
      );
      const generateEndTime = Date.now();

      // time it
      const drawStartTime = Date.now();
      draw(context, canvasWidth, canvasHeight, wasmResult);
      const drawEndTime = Date.now();

      const elapsed = generateEndTime - generateStartTime;
      console.log(`\tgenerate:wasm\tgenerate_elapsed: ${elapsed}[ms]`);
      console.log(
        `\tdraw: js\tdraw_elapsed: ${drawEndTime - drawStartTime}[ms]`
      );
    }
    let jsResult = null;
    {
      const CANVAS_ID = "canvas_js";
      let canvas = document.getElementById(CANVAS_ID);
      let context = canvas.getContext("2d");
      const canvasWidth = canvas.width;
      const canvasHeight = canvas.height;

      const generateStartTime = Date.now();
      jsResult = logic.generateMandelbrotSet(
        canvasWidth,
        canvasHeight,
        X_MIN,
        X_MAX,
        Y_MIN,
        Y_MAX,
        MAX_ITER
      );
      const generateEndTime = Date.now();

      const drawStartTime = Date.now();
      draw(context, canvasWidth, canvasHeight, jsResult);
      const drawEndTime = Date.now();

      const elapsed = generateEndTime - generateStartTime;
      console.log(`\tgenerate:js\tgenerate_elapsed: ${elapsed}[ms]`);
      console.log(
        `\tdraw: js\tdraw_elapsed: ${drawEndTime - drawStartTime}[ms]`
      );
    }
    {
      // 結果が等しいか確認
      let isSame = true;
      for (let i = 0; i < wasmResult.length; i++) {
        if (wasmResult[i] !== jsResult[i]) {
          console.log(i, wasmResult[i], jsResult[i]);
          isSame = false;
          break;
        }
      }
      console.log(`\n(wasmResult === hsResult):${isSame}`);
    }
  });
});

/**
 * 速度比較のため JS でもマンデルブロ集合を計算するロジックを実装
 */
const logic = {
  getNDiverged: function (x0, y0, max_iter) {
    let xn = 0.0;
    let yn = 0.0;
    // https://2ality.com/2018/12/creating-arrays.html#filling-arrays-with-undefined:
    // Array.from({ length: max_iter }, (x, i) => {});
    for (let i = 1; i < max_iter; i++) {
      let x_next = xn * xn - yn * yn + x0;
      let y_next = 2.0 * xn * yn + y0;
      xn = x_next;
      yn = y_next;
      if (xn * xn + yn * yn > 4.0) {
        return i;
      }
    }
    return max_iter;
  },
  generateMandelbrotSet: function (
    canvas_w,
    canvas_h,
    x_min,
    x_max,
    y_min,
    y_max,
    max_iter
  ) {
    let data = [];
    for (let i = 0; i < canvas_h; i++) {
      let y = y_min + ((y_max - y_min) * i) / canvas_h;
      for (let j = 0; j < canvas_w; j++) {
        let x = x_min + ((x_max - x_min) * j) / canvas_w;
        let iter_index = this.getNDiverged(x, y, max_iter);
        let v = (iter_index % 8) * 32; // 8 色に塗り分ける
        data.push(v); //R
        data.push(v); //G
        data.push(v); //B
        data.push(255); //A
      }
    }
    return data;
  },
};
