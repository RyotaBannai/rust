fn get_n_diverged(x0: f64, y0: f64, max_iter: usize) -> u8 {
  // 複素数 z_n の実部 xn, 虚部を yn とする
  let mut xn = 0.0;
  let mut yn = 0.0;
  for i in 1..max_iter {
    let x_next = xn * xn - yn * yn + x0;
    let y_next = 2.0 * xn * yn + y0;

    xn = x_next;
    yn = y_next;

    if yn * yn + xn * xn > 4.0 {
      return i as u8; // 複素数の絶対値が 2 を超えると発散すると判定
    }
  }
  max_iter as u8
}

pub fn generate_mandelbrot_set(
  canvas_w: usize,
  canvas_h: usize,
  x_min: f64,
  x_max: f64,
  y_min: f64,
  y_max: f64,
  max_iter: usize,
) -> Vec<u8> {
  let canvas_w_f64 = canvas_w as f64;
  let canvas_h_f64 = canvas_h as f64;

  // JS の 8 ビット符号なし整数の配列である Unit8ClampedArray型を作りため、Vec<u8> で色情報を作る
  let mut data = vec![];
  for i in 0..canvas_h {
    let i_f64 = i as f64;
    let y = y_min + (y_max - y_min) * i_f64 / canvas_h_f64;
    for j in 0..canvas_w {
      let x = x_min + (x_max - x_min) * j as f64 / canvas_w_f64;
      let iter_index = get_n_diverged(x, y, max_iter);
      let v = iter_index % 8 * 32; // 8 色に塗り分ける
      data.push(v); //R
      data.push(v); //G
      data.push(v); //B
      data.push(255); //A
    }
  }
  data
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_get_diverged() {
    let max_iter = 10;
    assert_eq!(get_n_diverged(1.0, 0.0, max_iter), 3); // 0 -> 1 -> 2 -> 5 で発散
    assert_eq!(get_n_diverged(0.0, 0.0, max_iter), max_iter as u8);
    assert_eq!(get_n_diverged(0.0, 1.0, max_iter), max_iter as u8);
  }

  #[test]
  fn test_generate_mandelbrot_set() {
    let canvas_w = 2;
    let canvas_h = 2;
    let x_min = -1.0;
    let x_max = 1.0;
    let y_min = -1.0;
    let y_max = 1.0;
    let max_iter = 8;
    // (x, y) in [(-1, -1), (-1, 0), (0, -1), (0, 0)] について値を確認
    assert_eq!(
      generate_mandelbrot_set(canvas_w, canvas_h, x_min, x_max, y_min, y_max, max_iter),
      vec![96, 96, 96, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]
    );
  }
}
