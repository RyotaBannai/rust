use actix_web::ResponseError;
use askama;
use r2d2;
use rusqlite;
use thiserror::Error;

// actix_web::ResponseError として使うために derive マクロで Debug を付与している必要がある。
#[derive(Error, Debug)]
pub enum AppError {
  #[error("Failed to render HTML")]
  AskamaError(#[from] askama::Error),

  #[error("Failed to get connection")]
  ConnectionPoolError(#[from] r2d2::Error),

  #[error("Failed to SQL execution")]
  SQLiteError(#[from] rusqlite::Error),
}

// actix_web::ResponseError を AppError に実装
// デフォルトのものを使用するので追加実装はない
impl ResponseError for AppError {}
