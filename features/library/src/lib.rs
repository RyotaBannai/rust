// フィーチャ parallel が有効になるときだけ有効になるコード
// 反対に有効にしないと使うことがきない
#[cfg(feature = "parallel")]
pub fn parallel() {
    println!("parallel is enabled");
}

#[cfg(feature = "serde")]
pub fn serde() {
    println!("serde is enabled");
}

#[cfg(feature = "special")]
pub fn special() {
    println!("special is enabled");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel() {
        // assert_eq!(2 + 2, 4);
        parallel();
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        // assert_eq!(2 + 2, 4);
        serde();
    }

    // Cargo.toml で default で設定していないため、有効のときだけテストするように修正
    // special を有効にする場合は、cargo の引数を次のようにする
    // $ cargo test --features special
    // default features を無効にしたい場合は、cargo test --no-default-features とする
    #[cfg(feature = "special")]
    #[test]
    fn test_special() {
        // assert_eq!(2 + 2, 4);
        special();
    }
}
