use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug, Error)]
pub enum AppError {
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

// check the lastest. https://docs.rs/tokio/1.2.0/tokio/net/struct.TcpStream.html
// tokio office impl sample. https://github.com/tokio-rs/tokio/blob/master/examples/tinyhttp.rs#L239
// tokio を使うと std の実装より楽 http://steavevaivai.hatenablog.com/entry/2020/09/13/173117
// good reference: https://tech-blog.optim.co.jp/entry/2019/11/08/163000#%E3%81%BE%E3%81%9A%E6%9C%80%E5%88%9D%E3%81%AB%E4%BC%9D%E3%81%88%E3%81%9F%E3%81%84%E3%81%93%E3%81%A8
#[tokio::main]
pub async fn run_client() -> Result<(), AppError> {
  // Connect to a peer
  let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

  // Write some data.
  stream.write_all(b"hello world!").await?;

  Ok(())
}
