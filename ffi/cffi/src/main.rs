// C で書かれたプログラムを Rust で使うことを宣言
extern "C" {
    fn c_hello();
}

fn main() {
    println!("Hello, world from Rust!");
    unsafe {
        c_hello();
    }
}
