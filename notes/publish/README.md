### Cargo.lock

- crate をインストールする時デフォルトで、cargo.lock を無視し、依存するパッケージはできるだけ新しいもので作成される。これだと破壊的な回収が行われたときに動作しなくなる可能性があるため、`cargo install --locked` でビルドされた時のパッケージバージョンでインストールする。(`--frozen` も同じ)
- `cargo install --path [local path]` -> Filesystem path to local crate to install.

- more info https://qiita.com/dalance/items/759b85362fdbf80eb23c
