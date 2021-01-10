#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod module_a; // 有効化
pub mod module_b; // ライブラリとして作成している場合は、lib で pub をつけないと、private になるため使用できな
