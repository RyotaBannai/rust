pub fn hello() {
    println!("Hello");
    let mut a = 0;
    let mut b = 0;

    // cargo clippy warning.
    a = b;
    b = a;
}
