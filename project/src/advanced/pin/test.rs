use pin_utils::pin_mut;
use std::marker::PhantomPinned;
use std::pin::Pin;

struct SelfRef {
  x: u32,
  ptr: *const u32,
}
// まず、Pinの仕組みは値をムーブさせたくないときに必要になるもの.
// 「値をムーブさせたくないとき」というのは、主に自己参照構造体を使うときのこと.

impl SelfRef {
  pub fn new(x: u32) -> Self {
    let mut this = Self {
      x,
      ptr: std::ptr::null(),
    };
    this.ptr = &this.x;
    assert_eq!(&this.x as *const _, this.ptr); // この時点ではアドレスは変わらないのでテストが成功する
    this // ここで値を返した瞬間にxのアドレスが変わり、ptrの値が不正となる.
  }
}
// > 先程のnewメソッドは、SelfRefを生成した後にその値を返そうとしていました。これは、一度newメソッドのスタック領域にSelfRefを生成し、それを別のメモリ領域にムーブすることになります。ところが、参照bの実態は変数aを指すポインタであり、ムーブが行われるとaが置かれるメモリアドレスは変化してしまいます。これによりnewメソッドが終了すると参照bは無効となってしまいます。これが自己参照構造体が作れない基本的な理由です。

// > aの実体はヒープ領域にあるので、bがaを指している場合にムーブが起きても、参照が壊れることはなさそうです。しかしながら、このような自己参照構造体も作ることができません。もし作れたとして、このSelfRefへのmutableな参照が存在した場合、bの参照が生きているにも関わらずaを直接書き換えることが出来てしまいます。これは「ある変数へのmutableな参照をもつ場合、それが唯一の参照である」というRustのルールを破りますし、またaに別のBoxを代入することで、bが無効な参照になってしまいます。

pub fn test_ptr_ne() {
  let v = SelfRef::new(0);
  assert_eq!(&v.x as *const _, v.ptr);
}
// 「ムーブを許容しない型」には自分でその型マークを付ける必要がある
// これに std::marker::PhantomPinned を使う
struct NotUnpin {
  _pinned: PhantomPinned,
}

impl NotUnpin {
  pub fn new() -> Self {
    Self {
      _pinned: PhantomPinned,
    }
  }

  // fn method(&self) = fn method(self: &Self)
  // self を Pin として受け取った場合、それはピン留めされた参照であるため、その中身は構造体の外ではムーブされないことになる
  pub fn method(self: Pin<&mut Self>) {
    println!("Pinned!")
  }
}
// 値が Pin で包まれているかをコンパイル時に確認するためのダミー関数
fn assert_pin<T>(_: &Pin<&mut T>) {}

pub fn test_pin_utils() {
  // スタックから移動しない変数を作成
  // スタックから移動しない変数 -> 変数に同じ名前の変数の参照を代入して、変数の参照先を変更できなくする（std::mem::replaceやstd::mem::swap を使うと変更できてしまう）
  // https://tech-blog.optim.co.jp/entry/2020/03/05/160000#%E3%82%B9%E3%82%BF%E3%83%83%E3%82%AF%E3%81%8B%E3%82%89%E7%A7%BB%E5%8B%95%E3%81%97%E3%81%AA%E3%81%84%E5%A4%89%E6%95%B0
  let obj = NotUnpin::new();

  // obj は Unpin を実装していないため Pin::new を使えない
  // let obj = Pin::new(obj);

  // pin_mut よってスタックにピン留め
  pin_mut!(obj);
  // objは Pin<&mut T> である -> Pin で包まれている
  assert_pin::<NotUnpin>(&obj);

  obj.as_mut().method();
  obj.as_mut().method();
}

pub fn test_tokio_pin() {
  // tokio の pin! 使うと pin_utils と同等の処理を書くことができるが、crate サイズが大きいため tokio  を使わない時は pin_utils の方が良い.advanced
  // use tokio::pin;
  // pin_utils::pin_mutと同じ使い方
  // {
  //   let obj = NotUnpin::new();
  //   pin!(obj);
  //   obj.as_mut().method();
  // }
  // その場で変数の宣言も出来る
  // {
  //   pin! {
  //       let obj = NotUnpin::new();
  //   }
  //   obj.as_mut().method();
  // }
}

pub fn test_box_pin() {
  let obj = NotUnpin::new();
  // Box::pinによってヒープでピン留めする
  let mut obj: Pin<Box<NotUnpin>> = Box::pin(obj);
  // Pinになったのでメソッドを呼び出せる
  // selfの型をPin<Box<Self>>ではなくPin<&mut Self>にしているため、obj.method()として呼び出せない
  // 代わりにPin::as_mutを使いPin<Box<T>>からPin<&mut T>に変換して呼び出す
  obj.as_mut().method();
}

// Rc::pin / Arc::pin
// Rc::pin 及び Arc::pin は、Box::pin と同じく変数をヒープに固定して Pin でピン留めする。
// 参照カウントが必要な場面で使うことになる

pub fn main() {
  // これらの仕組みは全てスタックかヒープに変数を固定し、Pinにピン留めする機能
  test_pin_utils();
  test_box_pin();
}
