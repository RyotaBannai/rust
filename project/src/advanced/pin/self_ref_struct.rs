use owning_ref::OwningRef;

// 自己参照構造体が作れない理由は、Rustのライフタイムの制限という以前に、
// メモリとポインタの仕組みそのものにある

#[derive(Debug)]
struct SelfRef<'a> {
  a: i32, //  Box<i32> ヒープ領域に作ってもエラー: b の参照が生きているにも関わらず a を直接書き換えることが出来てしまう。これは「ある変数への mutable な参照をもつ場合、それが唯一の参照である」という Rust のルールを破るし、a に別の Box を代入することで、b が無効な参照になる
  b: &'a i32, // 参照なので lifetime をつける
}

// impl<'a> SelfRef<'a> {
//   fn new() -> Self {
//     let a = 0i32;
//     Self { a, b: &a }
//   }
// }

struct Parent {
  i: i32,
}

struct Child<'a> {
  parent: &'a Parent,
}

// Child は Parent の lifetime ないでしか存在できない
impl<'a> Child<'a> {
  fn new(parent: &'a Parent) -> Self {
    Child { parent }
  }
}

struct SelfRef2<'a> {
  parent: Parent,
  child: Child<'a>,
}

// impl<'a> SelfRef2<'a> {
//   fn new() -> Self {
//     let parent = Parent { i: 0 };
//     let child = Child::new(&parent);
//     Self { parent, child } // fails at here.
//     // self_ref_struct.rs(40, 28): `parent` is borrowed here
//     // self_ref_struct.rs(41, 5): returns a value referencing data owned by the current function
//   }
// }

fn return_owned_and_referenced() -> OwningRef<Vec<u8>, [u8]> {
  let v = vec![0, 1, 2];
  let or = OwningRef::new(v);
  let or = or.map(|v| &v[1..2]);
  or
}

pub fn test() {
  // let sr = SelfRef::new();
  // dbg!(sr);

  let or = return_owned_and_referenced();
  dbg!(&or); // owning_ref struct
  dbg!(&*or); // reference
  dbg!(or.as_owner()); // owner
}
