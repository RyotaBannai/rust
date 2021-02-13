# rust

### About

- The playground for Rust

### Project を実行パスへインストール

- `cargo install --path .`

### Different between Eq and PartialEq

- El を満たすためには, `反射律(reflective)`、`対称律(symmetric)`、`推移律(transitive)`を満たす必要がある。これは Java の Object@equals メソッドをオーバーライドする時の条件と同じ。
- ただ常に `反射律`を満たすことはできない。例えば, f32 の NaN(Not a Number) は Nan == Nan => false となってしまう。この`反射律`の条件だけは満たさなくても良いとしたものが `PartialEq`. `PartialOrd` も同様.
- NaN が含まれている Vec<i32> をソートすると `unwarp パニック`になる

### 静的ディスパッチ、動的ディスパッチ

- インスタンスからメソッドを呼び出す場合、コンパイル時にどのインスタンスからメソッドが呼び出されるかわかっているため処理を高速化できる → 「静的ディスパッチ」
- どのインスタンスから呼び出されるかわからない場合、実行時に決める → 「動的ディスパッチ」 動的ディスパッチの例としては以下のような場合で、for で繰り返し処理をするような際に、tweet() メソッドを呼び出した時にコンパイル時にはどのインスタンスかは分からない。この「静的ディスパッチ」では解決できない場合は `dyn` を使って動的ディスパッチを使う.

```rust
let dove = Dove{}
let duck = Duck{}
let birds: Vec<Box<dyn Tweet>> = vec![Box::new(dove), Box::new(duck)];
for bird in birds{
  bird.tweet()
}
```

### マーカトレイト

- `Send`: スレッド間をまたいで所有権を転送（委譲）できることを示す
  - スレッド間を転送してはいけない型を転送しようとするとコンパイルエラーとなり間違った方の仕様を防ぐことができる
- `Sync`: 複数のスレッドから安全にアクセスできることを示す
  - 例えば、`Mutex` は `lock` メソッドによる排他制御によって複数のスレッドから安全にアクセスできるので、Sync を実装している
  - `Sync` を実装していない型を複数のスレッドからアクセスするとコンパイルエラーになる

### Future

- タスクが作成された時点では実行されておらず、ランタイム（非同期ランタイム： Future の実行タイミングを制御し、必要なタイミングで処理を走らせる）に乗った時点でスケジューリングされ、実行される。
  - 「実行」するかどうかの判断は、 `poll()` （ポーリング）によってチェックされる。
  - チェックする主体は `Waker(std::task::Waker)` であり poll 関数の引数として渡される `Context` 内にラップされている。
  - `Poll::Pending` が返されると `poll()` はまた`他のタスクが実行状態でなくなる`まで呼ばれず、他のタスクを実行する。（他のタスクを処理した後に再度 poll する）
  - `Poll::Ready<T>` が返されるとタスクが実行完了となり、ランタイムは次の実行状態に移る。
  - このポーリングを繰り返しながら実行するランタイムのことを `Executor` と呼ぶ。
- `async fn => impl Future のシンタックスシュガー`:

```rust
async fn some_func(in: i32) -> i32{
  other_func().await;
}
```

- 上記の `async` は次のように記述することもできる

```rust
fn some_fund(in: i32) -> impl Future<Output = i32>{
  async {
    other_func().await;
  }
}
```

### ライフタイム

- [Reference](https://doc.rust-lang.org/1.9.0/book/lifetimes.html)
- `Dangling pointer` => ` use after free`

```
・I acquire a handle to some kind of resource.
・I lend you a reference to the resource.
・I decide I’m done with the resource, and deallocate it, while you still have your reference.
・You decide to use the resource.
```

- Uh oh! Your reference is pointing to an invalid resource. This is called `a dangling pointer` or `use after free`, when the resource is memory. To fix this, we have to make sure that step four never happens after step three. When we have a function that `takes an argument by reference`, we can be implicit or explicit about the lifetime of the reference:

```rust
// implicit
fn func(x: &i32) {}
// explicit
fn func<'a>(x: &'a i32){}
```

- The `'a` reads `the lifetime a`. Technically, every reference has some lifetime associated with it, but `the compiler lets you elide` (i.e. omit, see `Lifetime Elision`) them in common cases.
- If you compare `&mut i32` to `&'a mut i32`, they’re the same, it’s that the lifetime `'a` has snuck in between the `&` and the `mut i32`.
- We read

  - `&mut i32` as `a mutable reference to an i32`, and
  - `&'a mut i32` as `a mutable reference to an i32 with the lifetime 'a`

#### In structs

- You'll also need explicit lifetimes when working with structs that contains references:

```rust
struct Foo<'a> {
  x: &'a i32,
}
fn main(){
  let y = &5;
  left f = Foo{ x: y};
  println!("{}", f.x);
}
```

#### `So why do we need a lifetime here?`

- -> We need to ensure that any reference to a `Foo` cannot outlive the reference to an i32 it contains.

#### Multiple lifetimes

- In this example, x and y have different valid scopes, but the return value has the same lifetime as x.

```rust
fn x_or_y<'a, 'b>(x: &'a str, y: &'b str) -> &'a str{}
```

#### 'static

- The lifetime named `static` is a special lifetime: it signals that something has `the lifetime of the entire program`.
- String literals have the type `&'static str` because the reference is always alive: they are baked into `the data segment of the final binary`. Another example are globals.

```rust
let x: &'static str = "Hello, world.";
static FOO: i32 = 5;
let x: &'static i32 = &FOO;
```

#### Lifetime Elision (<-> expanded)

- `Three Rules`:

  - Each elided lifetime in a function’s arguments becomes a distinct lifetime parameter.
  - If there is exactly one input lifetime, elided or not, that lifetime is assigned to all elided lifetimes in the return values of that function.
  - If there are multiple input lifetimes, but one of them is ` &self` or `&mut self`, the lifetime of `self` is assigned to all elided output lifetimes.

#### lifetime examples from rust docs

```rust
fn print(s: &str); // elided
fn print<'a>(s: &'a str); // expanded

fn debug(lvl: u32, s: &str); // elided
fn debug<'a>(lvl: u32, s: &'a str); // expanded

// In the preceding example, `lvl` doesn’t need a lifetime because it’s not a
// reference (`&`). Only things relating to references (such as a `struct`
// which contains a reference) need lifetimes.

fn substr(s: &str, until: u32) -> &str; // elided
fn substr<'a>(s: &'a str, until: u32) -> &'a str; // expanded

fn get_str() -> &str; // ILLEGAL, no inputs

fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambiguous

fn get_mut(&mut self) -> &mut T; // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new(buf: &mut [u8]) -> BufWriter; // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded
```

#### 'static ライフタイム

- [Reference](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html)

##### 'static の２つの使用例

###### 1. A reference with 'static lifetime:

```rust
let s: &'static str = "string";
```

- As a reference lifetime `'static` indicates that the data pointed to by the reference `lives for the entire lifetime of the running program` -> in other words, the reference to `'static lifetime variable` can no longer be used, but the data remains in the binary. `(1)`
- `It can still be coerced to a shorter lifetime`. `(2)`
- There are two ways to make a variable with `'static` lifetime, and both are `stored in the read-only memory of the binary`:

1. make `a constant` with the static declaration

- There are `two different types of constants` which `can be declared in any scope including global`.

1. `const`: An unchangeable value(the common case)
2. `static`: A possibly `mut` able variable with `'static` lifetime.

- the static lifetime is inferred and does not have to be specified.
- Accessing or modifying a mutable static variable is `unsafe`.

2. make `a string literal` which has type: `&'static str`

```rust
// 1. make a constant
static NUM: i32 = 32;
// 2. make a string literal
let static_string = "string"
```

```rust
static NUM: i32 = 32;
fn coerce_staticM<'a>(_:&'a i32) -> &'a i32{
  &NUM
}

fn main(){
  // (1)
  {
    let static_string = "string";
    println!("{}", static_string);
  }
  // (2)
  {
    let lifetime_num = 9;
    let coerced_static = coerce_static(&lifetime_num);
    println!("{}", coerced_static);
  }
  println!("NUM: {} stays accessible!", NUM); // &NUM ok as well.
  // 関数内部で lifetime が短くなっても参照できるし、
  // グローバルだからブロックを出ても参照できる
}
```

2. 'static as part of `a trait bound`:

```rust
fn genericM<T>(x: T) where T: 'static{}
```

###### 2. Trait bound

- As a trait bound, it means `the type does not contain any non-static references`. Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it.

- It's important to understand this means that any owned data always passes `a 'static lifetime bound`, but a reference to that owned data generally does not(3):
  (any owned data can pass, but not a reference to an owned data...)

```rust
use std::fmt::Debug;

fn print_it( input: impl Debug + 'static )
{
    println!( "'static value passed in is: {:?}", input );
}

fn use_it()
{
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // use_it(), so it's not 'static:
    print_it(&i); // error 完全に所有権を渡さないといけない. (3)
}
```

##### Coercion

- [Reference](https://doc.rust-lang.org/rust-by-example/scope/lifetime/lifetime_coercion.html)
- `A longer lifetime can be coerced into a shorter one so that it works inside a scope it normally wouldn't work in`. This comes in the form of inferred coercion by the Rust compiler, and also in the form of declaring a lifetime difference:
- lifetime が異なる引数を渡すことはできないが、lifetime が長い変数を短い変数に合わせてることで引数として扱うことができる.

```rust
// Here, Rust infers a lifetime that is as short as possible.
// The two references are then coerced to that lifetime.
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // Longer lifetime

    {
        let second = 3; // Shorter lifetime

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    };
}
```

##### Bounds

- [Reference](https://doc.rust-lang.org/rust-by-example/scope/lifetime/lifetime_bounds.html)

1. `T: 'a` :All references in T must outlive lifetime `'a`
2. `T: Trait + 'a` :Type `T` must implement trait `Trait` and all references in T must outlive `'a`

##### 型 T に 'static ライフタイム境界をつける場合の意図

- [reference](https://laysakura.github.io/2020/05/21/rust-static-lifetime-and-static-bounds/)

- `T には参照を含まない（ T が struct, enum, ベクタなどであった場合にはその中身も参照ではない）ことを要請する` （という使い方が大半. 3 番目のように struct や enum やベクタに 'static ライフタイムな参照を含めたくなるようなケースがあまりない（その場合は値そのものをフィールドにする））
  もう少し厳密にいうと、 `T: 'static` ならば、
- `T` がスカラ型の値である。（e.g. `T <- 123`）
- `T` が`複合型（struct, enum, ベクタ, 配列 など、アクセスできる内部構造を持つ型）の値`であり、その内部構造は参照を持たない。（Eg. `T <- struct S(u32)`, `enum E { V1(u32), V2(i32) }`, `T <- Vec<u32>`）
- `T` が複合型の値であり、その内部構造に `'static` ライフタイムの参照を含む。（Eg. `T <- struct S(u32, &'static u32)`, `T <- Vec<&'static str>`）
- `T` が、上記のいずれか値の `'static` ライフタイムの参照である。(Eg. `T <- &'static 123`, `T <- &'static S(u32)`)

####

- 通常の`非 static な`ライフタイムであれば、async 内におけるライフタイムはほとんど問題にならない.

```rust
async fn some_great_func(arg: &i32) -> i32{
  *arg
}
```

- rust のコンパイラは、上記の `some_great_func` はライフタイム `'a`をもち、戻り値が Future である関数に内部的に変換する

```rust
fn some_great_func<'a>(arg: &'a i32) -> impl Future<Output = i32> + 'a {
  *arg
}
```

- しかし、スレッドをまたいで Future の値を送りたくなった際、`'static` ライフタイムを用いる必要がある。

### Interior Mutability Pattern

- `The interior mutability pattern`: `Mutating the value inside an immutable value`

- `The borrowing rules`:
  - At any given time, you can have either (but not both of) `one mutable reference` or `any number of immutable references`.
  - References must always be valid.
- Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
  - `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
  - `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.
  - Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

### malloc/ free in Rust

- [Referece](https://mmi.hatenablog.com/entry/2017/08/07/191033)

### 非同期処理

- そもそも、Pin は非同期プログラミングを実現するために導入されたものである。 Rust 標準では、Unpin を実装しないオブジェクトは`非同期関数の戻り値`と`非同期ブロック`のみ

- 非同期関数などの「途中で中断し、再開できる関数」は CPU では表現できないため、 内部では`コルーチン`として表現され、その後コンパイラによって`ステートマシン`に変換される。 この`ステートマシン`が`自己参照構造体`になる

- [ref article](https://webcache.googleusercontent.com/search?q=cache:l353ZuE0188J:https://tech-blog.optim.co.jp/entry/2020/03/05/160000+&cd=3&hl=en&ct=clnk&gl=jp)
- [std::pin の勘所 LT](https://webcache.googleusercontent.com/search?q=cache:l353ZuE0188J:https://tech-blog.optim.co.jp/entry/2020/03/05/160000+&cd=3&hl=en&ct=clnk&gl=jp)

### Which problems does owning-ref solve?

- [ref](https://users.rust-lang.org/t/which-problems-does-owning-ref-solve/29245/4?u=ryotabannai)

### Fearless Concurrency

- (Concurrency/ Parallel - 並行性あるいは並列性)

- `所有権`と`型システム`は、 `メモリ安全性`と`並行性問題`を管理するために役に立つ一連の強力な道具である. なぜなら、`所有権`と`型チェック`を活用することで、多くの`並行性エラー`は、`実行時エラーではなくコンパイル時エラー`になり、`実行時に並行性のバグが起きた状況と全く同じ状況を再現しようと時間を浪費させるよりも、不正なコードはコンパイル時のエラーとなるためプロダクトになった後でなく作業中にコードを修正することができる`

### スレッドに関して

- プログラミング言語によってスレッドはいくつかの方法で実装されている。多くの OS で、新規スレッドを生成する API が提供されている。 `言語が OS の API を呼び出してスレッドを生成するこのモデル`を時に `1:1` と呼び、`1 つの OS スレッド`に対して `1 つの言語スレッド`を意味する。

- 多くのプログラミング言語はスレッドの独自の特別な実装を提供している。`プログラミング言語が提供するスレッド`は、`グリーンスレッド (Green threads)`として知られ、この`グリーンスレッド`を使用する言語は、`それを異なる数の OS スレッドの文脈で実行`します。 このため、`グリーンスレッドのモデル`は `M:N モデル`と呼ばる (`M 個のグリーンスレッド`に対して、 `N 個の OS スレッド`があり、`M と N は必ずしも同じ数字ではない`)
- **Green Threads more details**: オペレーティングシステムではなく仮想マシン (VM) によってスケジュールされるスレッドである。 グリーンスレッドは`ネイティブの OS の機能に依存せず`にマ`ルチスレッド環境をエミュレートする`

#### なぜ Go では何百万もの Goroutine を作れるのに Java は数千のスレッドしか作れないのか?

- [ref](https://mahata.gitlab.io/post/2018-10-15-goroutines-vs-java-threads/)

##### スタック問題

- JVM が使用する、OS のスレッドは、`スタックサイズが固定長`である。スタックサイズそのものは設定可能で、64 ビット環境での JVM デフォルトのスレッドあたりのスタック長は 1MB になるが、データ量がなくてもそれぞれの`ユーザー空間スレッド（プログラミング言語がスケジュールするスレッド、論理的なスレッドは OS が走らせられる、1:1 モデルを指す）`に固定のスタックサイズが割り振られてしまう。
- Go は独自のスケジューリング機構を実装しているのでスタックサイズは保存するデータに応じて動的に伸縮させることができる。端的にいうと新しい Goroutine はたかだか 4KB のスタックを持つことになる。ひとつのスタックあたり 4KB の容量なので、250 万もの Goroutine を 1GB の RAM に収められる。ひとつのスレッドあたりに 1MB を要する Java と比べると大きな違いになる。

##### スレッド問題

- Java(JVM) はスレッディングを可能な限り OS のスレッドに委譲している。スレッドは 1:1 (`1 つの OS スレッド`に対して`1 つの言語スレッド`)なので、スレッドの切り替えが必要になる。この切り替えには、`コンテキストスイッチ`という機能を使用するが、1~100 マイクロ秒くらいの時間がかかる。(-> 1 秒で全てのスレッドに処理させようとするとたかだか 1 コアの CPU に 100k スレッドしか走らせられないということになる。しかも、スレッドが実際の仕事をする時間は別にかかる。)
- Go は複数のスレッドを使用せずに一つのスレッド上で複数の Goroutine を走らせるために独自のスケジューラを実装している。コンテキストスイッチに遅延が発生しない。さらに、ヒマな Goroutine （channel が空）は自身の OS のスレッドに留める。このことにより、アクティブな Goroutine がひとつのスレッドに割り当てられ、大多数のヒマな Goroutine が別に管理される。これによって、遅延を軽減させることができる。

- Java でもこの`ユーザー空間で`いつスレッドが仕事をできるかを管理する`ランタイムスケジューラ`を作ることは可能で、これは [`Akka`](https://akka.io/) のようなフレームワークが実現している。ランタイムスケジューラは数万のアクターをサポートする機構の基礎となる。
