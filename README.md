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
