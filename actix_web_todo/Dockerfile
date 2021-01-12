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

# ビルドした生成物から、空のアプリケーションを削除
RUN rm -f target/release/deps/todo*

# 実際に必要なソースを copy してビルド
COPY ./src ./src
COPY ./templates ./templates
RUN cargo build --release 

# 新しくリリース用のイメージを用意（実行体だけを入れる）
FROM rust:1.43

COPY --from=builder /tood/target/release/todo /usr/local/bin/todo
CMD ["todo"]