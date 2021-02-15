use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug, Error)]
pub enum AppError {
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

// check the lastest. https://docs.rs/tokio/1.2.0/tokio/net/struct.TcpStream.html
#[tokio::main]
pub async fn run_client() -> Result<(), AppError> {
  // Connect to a peer
  let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

  // Write some data.
  stream.write_all(b"hello world!").await?;

  Ok(())
}
