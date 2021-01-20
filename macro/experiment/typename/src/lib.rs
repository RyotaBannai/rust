pub use typename_derive::TypeName;

// マクロ名とトレイト名は同じであっても問題ない
// X という derive マクロによって X というトレイトが導出される
// そうすると、main では TypeNameTrait を import しなくて良い

pub trait TypeNameTrait {
    fn type_name(&self) -> &str;
}
