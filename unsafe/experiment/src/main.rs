use std::ptr::{read, replace, write};

pub fn replace_with<T, F>(r: &mut T, f: F)
where
    F: FnOnce(T) -> T,
{
    let value = f(unsafe { read(r) });
    unsafe { write(r, value) };
}

pub fn test_free_twice() {
    let mut s = String::from("hello");
    replace_with(&mut s, |s| s + ", world!");

    // メモリの二重解放
    // replace_with(&mut s, |_| panic!()); // experiment(42763,0x10f74be00) malloc: *** error for object 0x7f9591c05e60: pointer being freed was not allocated
    // ・T: Default を要求するか t: T を受け取りそれを使って replace するか, take を使って値を取り出してクロージャに渡す、または
    // ・unsafe fn にして、渡されたクロージャがパニックしないことを呼び出しがわに厳守させる
    println!("{}", s);
}

pub fn is_ptr_null(ptr: *const u8) {
    if ptr.is_null() {
        eprintln!("prt is null");
    } else {
        eprintln!("prt is not null");
    }

    // dereference of raw pointer is unsafe and requires unsafe function or block
    eprintln!("*ptr is {}", unsafe { *ptr });
}

pub fn test_pass_ptr() {
    let mut hello = "Hello";
    let hello_ptr = hello.as_ptr();
    is_ptr_null(std::ptr::null());
    is_ptr_null(hello_ptr);
}

use std::mem::size_of_val;
use std::slice::from_raw_parts;

pub fn anything_as_bytes<T: ?Sized>(val: &T) -> &[u8] {
    unsafe { from_raw_parts(val as *const T as *const u8, size_of_val(val)) }
}

pub fn test_violate_aliasing() {
    let cell = std::cell::Cell::new(43);
    let bytes = anything_as_bytes(&cell);
    // bytes は cell（A mutable memory location/ 内部可変性コンテナ）を参照しているが、
    // set によって書き換えられてしまうため、最終的な結果も変わってしまう。
    // → UB なので unsafe fn にする
    // また、パディングがある struct もこのような処理をすると、意図した通りデータのメモリ分を取れなくてい場合があるため、
    // UB と考えられている

    cell.set(42);
    eprintln!("bytes = {:x?}", bytes);
    println!("cell value is {}", std::str::from_utf8(bytes).unwrap());
}

fn main() {
    test_violate_aliasing();
}
