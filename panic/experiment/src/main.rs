use std::panic::catch_unwind;

// ナイーブな最大値と最小値の差を出力する処理
fn print_range(x: &[i32]) {
    let min = x.iter().min().unwrap();
    let max = x.iter().max().unwrap();
    eprintln!("max - min = {}", max - min);
}

fn main() {
    // 2 types of bugs
    let requests = vec![
        vec![1, 2, 3],
        vec![],
        vec![2147483647, -2147483647],
        vec![0, 42],
    ];

    for request in &requests {
        // catch_unwind でエラーが起きても loop を続ける
        let result = catch_unwind(|| print_range(request));
        if let Err(_payload) = result {
            eprintln!("***** print_range failed *****");
        } else {
            eprintln!(">success");
        }
    }
}
