use actix_web::ResponseError;
use askama;
use thiserror::Error;

// actix_web::ResponseError として使うために derive マクロで Debug を付与している必要がある。
#[derive(Error, Debug)]
pub enum AppError {
  #[error("Failed to render HTML")]
  AskamaError(#[from] askama::Error),
}

// actix_web::ResponseError を AppError に実装
// デフォルトのものを使用するので追加実装はない
impl ResponseError for AppError {}
