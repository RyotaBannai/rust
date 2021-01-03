# rust

The playground for Rust

### Different between Eq and PartialEq

- El を満たすためには, `反射律(reflective)`、`対称律(symmetric)`、`推移律(transitive)`を満たす必要がある。これは Java の Object@equals メソッドをオーバーライドする時の条件と同じ。
- ただ常に `反射律`を満たすことはできない。例えば, f32 の NaN(Not a Number) は Nan == Nan => false となってしまう。この`反射律`の条件だけは満たさなくても良いとしたものが `PartialEq`. `PartialOrd` も同様.
- NaN が含まれている Vec<i32> をソートすると `unwarp パニック`になる
