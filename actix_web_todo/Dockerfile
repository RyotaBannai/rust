FROM rust:1.43
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
RUN rm -r target/release/deps/todo*

# 実際に必要なソースを copy してビルド
COPY ./src ./src
COPY ./templates ./templates
RUN cargo build --release 

RUN cargo install --path .
CMD ["todo"]