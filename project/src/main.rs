mod archive;
#[path = "result/result.rs"]
mod result;
mod utils;
use archive::my_trait::birds::Tweet;

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
fn main() {
    // result::test_result3();
    // utils::nested::func();
    // archive::my_vec::test_vec();
    // archive::impl_iterator::test_my_iter();
    // use_traited_struct();
    archive::my_trait::my_drop::use_droppable();
}
