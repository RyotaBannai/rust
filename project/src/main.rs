mod archive;
#[path = "result/result.rs"]
mod result;
mod utils;
use archive::experiment::test_static::main_fn;
use archive::multi_thread::{
    make_spawn::make_spawn, msg_passing::msg_passing, share_memory::share_memory,
};
use archive::my_trait::birds::Tweet;
use archive::test_future::{test_async_await::call_async_funcs, test_future::experiment};

fn cast_string() {
    let s1: String = String::from("Hello world");
    let s2: &str = &s1; // String -> &str
    let s3: String = s2.to_string(); // &str -> String
    println!("{}", s3);
}
fn use_traited_struct() {
    let dove = archive::my_trait::birds::Dove {};
    let duck = archive::my_trait::birds::Duck {};
    dove.tweet_twice();
    dove.shout();

    let birds_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
    for bird in birds_vec {
        bird.tweet();
    }
}

// https://stackoverflow.com/questions/32900809/how-to-suppress-function-is-never-used-warning-for-a-function-used-by-tests
#[allow(dead_code)]
fn list_dead_codes() {
    cast_string();
    utils::nested::func();
    archive::my_vec::test_vec();
    archive::impl_iterator::test_my_iter();
    use_traited_struct();
    archive::my_trait::my_drop::use_droppable();
    make_spawn();
    share_memory();
    msg_passing();
    experiment();
    call_async_funcs();
}

fn main() {
    main_fn();
}
