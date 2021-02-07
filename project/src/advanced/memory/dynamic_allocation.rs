use super::memory_allocation::print_info;
use std::alloc::{alloc, dealloc, Layout, LayoutErr};
use std::ops::{Deref, DerefMut};
use std::slice;

// struct で便利にしてみる
// ref https://qiita.com/moriai/items/67761b3c0d83da3b6bb5
#[derive(Debug)]
struct AlignedAlloc {
  ptr: *mut u8,
  layout: Layout,
}
impl AlignedAlloc {
  fn new(len: usize, align: usize) -> Result<AlignedAlloc, LayoutErr> {
    let layout = match Layout::from_size_align(len, align) {
      Ok(l) => l,
      Err(e) => Err(e)?,
    };
    unsafe {
      let ptr = alloc(layout);
      Ok(AlignedAlloc { ptr, layout })
    }
  }
}
impl Drop for AlignedAlloc {
  fn drop(&mut self) {
    unsafe {
      dealloc(self.ptr, self.layout);
    }
  }
}
impl Deref for AlignedAlloc {
  type Target = [u8];
  fn deref(&self) -> &[u8] {
    unsafe { slice::from_raw_parts(self.ptr, self.layout.size()) }
  }
}
impl DerefMut for AlignedAlloc {
  fn deref_mut(&mut self) -> &mut [u8] {
    unsafe { slice::from_raw_parts_mut(self.ptr, self.layout.size()) }
  }
}

const ONEGB: usize = 1024 * 1024 * 1024;

pub fn use_struct() {
  let mut size: usize = 4;
  // while ループを回るたびに mem0 と mem1 がスコープからを外れるので、
  // 領域が開放されて、再利用されていることが分かる
  while size <= ONEGB {
    let mut mem0 = match AlignedAlloc::new(size, 4096) {
      Ok(m) => m,
      Err(e) => {
        panic!("AlignedAlloc failed {:?}", e)
      }
    };
    print_info(false, "first", &mem0);

    let mut mem1 = match AlignedAlloc::new(size, 4096) {
      Ok(m) => m,
      Err(e) => {
        panic!("AlignedAlloc failed {:?}", e)
      }
    };
    print_info(false, "second", &mem1);

    size *= 4;
  }
}
