use futures::prelude::*;
use tokio::net::TcpListener;
use tokio_util::codec::{BytesCodec, Decoder};

use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

// この属性を指定することで非同期 main 関数を実行するランタイムを設定できる。これを指定しない時は自分でランタイムを起動する必要がある

#[tokio::main]
pub async fn run_server() -> Result<(), AppError> {
  let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
  // TCP 接続を受け付けるリスナーを作成
  let mut listener = TcpListener::bind(&addr).await?;
  println!("listening on: {}", addr);

  loop {
    // メインタスク上で新たな接続を受け入れる
    let (socket, _) = listener.accept().await?;

    tokio::spawn(async move {
      // クライアントから送られてくるデータを処理
      let mut framed = BytesCodec::new().framed(socket);
      // クライアントから送られてくるデータを受信 next() によって受信する
      // クライアントの接続が切れたら、next() は None を返す
      while let Some(message) = framed.next().await {
        match message {
          Ok(bytes) => println!("bytes: {:?}", bytes),
          Err(err) => println!("Socket closed with error: {:?}", err),
        }
      }
      println!("Socket received FIN packet and closed connection.")
    });
  }

  Ok(())
}
