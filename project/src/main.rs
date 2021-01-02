fn main() {
    let s1: String = String::from("Hello world");
    let s2: &str = &s1;                 // String -> &str
    let s3: String = s2.to_string();    // &str -> String
    println!("{}", s3);
}
