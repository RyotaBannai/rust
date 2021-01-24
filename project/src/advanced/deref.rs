use std::ops::Deref;

// ref https://qiita.com/emonuh/items/2607adfc7e7addd2a571

#[derive(Debug)]
struct Parent {
  value: String,
  child: Child,
}

impl Deref for Parent {
  type Target = Child;
  fn deref(&self) -> &Child {
    &self.child
  }
}

#[derive(Debug)]
struct Child {
  value: String,
  grand_child: Grandchild,
}

impl Deref for Child {
  type Target = Grandchild;
  fn deref(&self) -> &Grandchild {
    &self.grand_child
  }
}

#[derive(Debug)]
struct Grandchild {
  value: String,
}

impl Deref for Grandchild {
  type Target = String;
  fn deref(&self) -> &String {
    &self.value
  }
}

pub fn test() {
  let grand_child = Grandchild {
    value: "grand_child".to_owned(),
  };
  let child = Child {
    value: "child".to_owned(),
    grand_child: grand_child,
  };
  let parent = Parent {
    value: "parent".to_owned(),
    child: child,
  };

  //
  // 型 T を明示した変数への代入
  //
  // 検証1: 自動変換(Parent -> Child)
  let ref_child: &Child = &parent;
  println!("{:?}", ref_child); // Child { value: "child", grandchild: Grandchild { value: "grandchild" } }

  // 検証2: 再帰的に自動変換(Parent -> Child -> Grandchild)
  let ref_grandchild: &Grandchild = &parent;
  println!("{:?}", ref_grandchild); // Grandchild { value: "grandchild" }

  // 検証3: 再帰的に自動変換(Parent -> Child -> Grandchild->String)
  let ref_value: &String = &parent;
  println!("{:?}", ref_value); // "grandchild"

  // 検証4: 型を明示しない場合は通常の参照取得
  let ref_parent = &parent;
  println!("{:?}", ref_parent);

  //
  // 型 U から型 T が持つメソッドの呼び出し
  //
  parent.assert_in_parent("parent");
  parent.assert_in_child("child");
  parent.assert_in_grand_child("grand_child");
}

impl Parent {
  fn assert_in_parent(&self, expect: &str) {
    assert_eq!(&self.value, expect)
  }
}

impl Child {
  fn assert_in_child(&self, expect: &str) {
    assert_eq!(&self.value, expect)
  }
}
impl Grandchild {
  fn assert_in_grand_child(&self, expect: &str) {
    assert_eq!(&self.value, expect)
  }
}
