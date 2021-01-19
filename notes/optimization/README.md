### Remove biarny size

- check out for more details: https://github.com/johnthagen/min-sized-rust

#### LTO aka Link Time Optimization

- アプリケーションで使用されている crate どうしで同じパッケージを使っている場合、重複したリンクを排除してサイズを削減する. `Cargo.toml` に以下の追加をする

```toml
[profile.release]
lto = true
```

#### Cargo にバイナリサイズを小さくするように設定する

- リリースビルドでのバイナリの最適化には `s`, `z` オプションがあり、`Cargo.toml` に指定する必要がある

#### 並列度を下げる

- rust はデフォルトでビルドを並列化して高速に実行する。ただし、生成タスク（コードを並列に生成）をまたいで最適化することはできない。この並列化の`並列度`を下げることで最適化できる可能性がある。これも `Cargo.toml` に設定を入れる。

#### シンボルを取り除く

- シンボル（Symbols）を削除することで実行体をさらに小さくすることができる
  - `Symbols`: By default on Linux and macOS, symbol information is included in the compiled `.elf` file. This information is not needed to properly execute the binary.
  - `strip target/release/min-sized-rust` で削除
  - また、nightly (1.45.0-nightly (2020-05-28)) ではオプションとして削除できる。`cargo +nightly build -Z strip=symbols`
