mod advanced;
mod archive;
#[path = "result/result.rs"]
mod result;
mod test_type;
mod utils;
use archive::experiment::test_static::main_fn;
use archive::multi_thread::{
    make_spawn::make_spawn, msg_passing::msg_passing, share_memory::share_memory,
};
use archive::my_trait::birds::Tweet;
use archive::test_future::{test_async_await::call_async_funcs, test_future::experiment};
use utils::sub::{general::test_two_string_type, nested::func};

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
    func();
    archive::my_vec::test_vec();
    archive::impl_iterator::test_my_iter();
    use_traited_struct();
    archive::my_trait::my_drop::use_droppable();
    make_spawn();
    share_memory();
    msg_passing();
    experiment();
    call_async_funcs();
    main_fn();
    test_two_string_type();
    advanced::sized::check_byte_size();
    advanced::sized::get_pointer();
    advanced::sized::test_all_pointer_size_is_the_same();
    advanced::sized::get_pointer();
    advanced::sized::check_byte_string();
    advanced::deref::test();
    advanced::rc::test_rc();
    advanced::arc::share_data_from_multi_threads();
    advanced::refcell::test_refcell();
    advanced::rc_refcell::test();
    advanced::memory_leak::test();
    advanced::fix_memory_leak::test();
    advanced::process::general::test();
    advanced::process::command::test();
    advanced::process::pass_message::test();
    advanced::maybeuninit::test();
    // advanced::memory::memory_allocation::check_boundary();
    // advanced::memory::memory_allocation::use_layout();
    // advanced::memory::dynamic_allocation::use_struct();
    // advanced::memory::bits::test();
    // advanced::memory::helper::test();
}

fn main() {
    // test_type::any::test();
    // archive::experiment::pass_fn_asarg::main();
    // advanced::pin::test::main();
    advanced::pin::test::main();
}
