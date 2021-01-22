mod sub;
use std::panic::catch_unwind;
use std::thread;
use sub::my_drop;

// ナイーブな最大値と最小値の差を出力する処理
fn print_range(x: &[i32]) {
    // get an array
    // iterate over an array with a for loop
    // slice is just a reference of a portion of an array without copying.
    let min = x.iter().min().unwrap(); // unwrap Some()
    let max = x.iter().max().unwrap();
    eprintln!("max - min = {}", max - min);
}

fn test_requests() {
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
            eprintln!("***** print_range fßailed *****");
        } else {
            eprintln!(">success");
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

//https://doc.rust-lang.org/std/thread/
// yield_now():
// For example when polling on a resource, it is common to check that it is available, and if not to yield in order to avoid busy waiting.
// Thus the pattern of yielding after a failed poll is rather common when implementing low-level shared resources or synchronization primitives.
// However programmers will usually prefer to use channels, Condvars, Mutexes or join for their synchronization routines, as they avoid thinking about thread scheduling.

fn thread_has_a_panic_handling() {
    // スレッドのパニックの有無は、JoinHandle::join の Result の中に入っている
    // => スレッド自体が catch_unwind と同等の機能を持っていると考えられる（thread では panic 処理をする必要がない）
    let t1 = thread::spawn(|| assert!(false));
    // eprintln! macro is used for io::stderr or progress messages.
    // join を呼ぶとそのスレッドが完了するまで、呼び出し側が wait 状態になる
    eprintln!("is_ok = {}", t1.join().is_ok()); // false

    let t2 = thread::spawn(|| assert!(true));
    eprintln!("is_ok = {}", t2.join().is_ok()); // true
}

#[allow(dead_code)]
fn unused_list() {
    test_requests();
    // print_type_of();
    thread_has_a_panic_handling();
}

fn main() {
    sub::use_droppable();
}
