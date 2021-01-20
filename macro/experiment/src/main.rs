// https://doc.rust-lang.org/stable/reference/macros-by-example.html
// 宣言型マクロ例
macro_rules! five_times {
    // expr(MacroFragSpec のひとつ)を指定すると、任意の式を引数にとることができる
    ($x: expr) => {
        5 * $x
    };
}

// block を追加してあげると、変数定義も可能になる
macro_rules! my_vec {
    ($x: expr) => {{
        let mut temp_vec = Vec::new();
        temp_vec.push($x);
        temp_vec
    }};
}

// *: 引数を複数個取得できるようにする
// $()* で囲んだ範囲を繰り返し処理
// , は、区切りとしてのカンマで $() にマッチした内容だけ $x から取り出せる
macro_rules! my_vecs {
    ($( $x: expr ),* ) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

// ty 型指定
macro_rules! my_vecs_match {
    ($x: ty) => {{ // タイプを渡した時は、そのタイプの vector を返すような pattern match macro
        let temp_vec: Vec<$x> = Vec::new();
        temp_vec
    }};
    ($( $x: expr ),* ) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
        temp_vec
    }};
}

use tomlstruct::tomlstruct;

tomlstruct! {
    [Hello]
    name = "hello",
    version = 1.0

    [Goodnight]
    name = "good night",
    version = 1.0
}

fn main() {
    // assert_eq!(25, five_times!(2 + 3));
    // 展開後
    // assert_eq!(25, five_times!(5 * (2 + 3)));

    println!("{:?}", five_times!(5));
    println!("{:?}", my_vec![0]);
    println!("{:?}", my_vecs![0, 1, 2]);

    println!("{:?}", my_vecs_match!(i32)); // []

    let _ = Hello {
        name: String::from("hello"),
        version: 1.0,
    };

    let _ = Goodnight {
        name: String::from("goodnight"),
        version: 1.0,
    };
}
