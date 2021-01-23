use std::ptr::{read, write};

pub fn replace_with<T, F>(r: &mut T, f: F)
where
    F: FnOnce(T) -> T,
{
    let value = f(unsafe { read(r) });
    unsafe { write(r, value) };
}

fn main() {
    let mut s = String::from("hello");
    replace_with(&mut s, |s| s + ", world!");

    // メモリの二重解放
    // replace_with(&mut s, |_| panic!()); // experiment(42763,0x10f74be00) malloc: *** error for object 0x7f9591c05e60: pointer being freed was not allocated
    println!("{}", s);
}
