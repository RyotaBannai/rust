FROM rust:1.43 AS builder
WORKDIR /todo

#
# 依存 crates は先にビルドしておき、毎回ビルドしないようにする
#

COPY Cargo.toml Cargo.toml
# ビルドするために何もしないソースを入れておく
RUN mkdir src 
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release 

# 実際に必要なソースを copy してビルド
COPY ./src ./src
COPY ./templates ./templates
# ビルドした生成物から、空のアプリケーションを削除
RUN rm -f target/release/deps/todo*
RUN cargo build --release 

# 新しくリリース用のイメージを用意（実行体だけを入れる）
FROM debian:10.4

COPY --from=builder /todo/target/release/actix_web_todo /usr/local/bin/actix_web_todo
CMD ["actix_web_todo"]