# rust

The playground for Rust

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

- You'll also neeed sxplitic lifetimes when working wiht structs that contains references:

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
- String literals have the type &'static str because the reference is always alive: they are baked into `the data segment of the final binary`. Another example are globals.

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
