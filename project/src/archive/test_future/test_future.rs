use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(i32);

impl Future for CountDown {
  type Output = String;
  fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
    /*
      poll が呼ばれるごとに self.0 == 0 になるまでカウントダウン
      self.0 != 0 の場合は、再度 poll が必要なので、wake_by_ref() を呼び出して、Poll::Pending を返す
    */
    if self.0 == 0 {
      Poll::Ready("Zero!".to_string())
    } else {
      println!("{}", self.0);
      self.0 -= 1;
      cx.waker().wake_by_ref();
      Poll::Pending
    }
  }
}

pub fn experiment() {
  let countdown_future1 = CountDown(10);
  let countdown_future2 = CountDown(20);

  let cd_set = join_all(vec![countdown_future1, countdown_future2]);
  let res = executor::block_on(cd_set); // block_on は渡した Future が完了になるまでブロックしてまつメソッド
  for (i, s) in res.iter().enumerate() {
    println!("{}: {}", i, s);
  }
}
