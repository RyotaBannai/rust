use super::helper::{count_bits, count_bits_val, count_ones, int_bits, print_bits};

pub fn print_info(show_bits: bool, msg: &str, mem: &[u8]) {
  let addr = (&mem[0] as *const u8) as u64;
  let mut bound: u64 = 1;
  while addr & bound == 0 {
    bound <<= 1;
  }

  if show_bits {
    // show how to check the head of address when allocating a memory
    // head memory is like XXXXXXXXXXX1000
    // so compare until & op matches to 1(at some bit), which means not zero.
    print_bits(count_bits_val(&addr) as u8, addr as usize);
    print_bits(count_bits_val(&bound) as u8, bound as usize);
    print_bits(count_bits_val(&bound) as u8, (addr & bound) as usize);
  }

  // bound はメモリブロックの先頭アドレスが、何バイト境界に乗っているかを示している
  println!(
    "{:>6} > size: {:>10} addr: 0x{:>012x} bound: {:>10}",
    msg,
    mem.len(),
    addr,
    bound
  );
}

const ONEGB: usize = 1024 * 1024 * 1024;

pub fn check_boundary() {
  let mut size: usize = 2;
  while size <= ONEGB {
    let mut mem0: Vec<u8> = Vec::with_capacity(size);
    unsafe { mem0.set_len(size) }
    print_info(true, "first", &mem0);

    let mut mem1: Vec<u8> = Vec::with_capacity(size);
    unsafe { mem1.set_len(size) }
    print_info(true, "second", &mem1);

    size *= 2;
  }
}

use std::alloc::{alloc, Layout};
use std::slice;

fn aligned_alloc(size: usize) -> &'static mut [u8] {
  // std::alloc::alloc() でアロケーション時のレイアウトを指定できる
  // レイアウトは struct std::alloc::Layout で指定
  unsafe {
    let layout = Layout::from_size_align(size, 4096).unwrap();
    // heap memory 確保
    let raw_mem = alloc(layout);
    // https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
    slice::from_raw_parts_mut(raw_mem, size)
  }
}

pub fn use_layout() {
  // 4KB境界に乗っている
  // drop する手段が提供されず、ライフタイムが static なので、次々と新しい領域がアロケートされる
  let mut size: usize = 4;
  while size <= ONEGB {
    let mut mem0 = aligned_alloc(size);
    print_info(false, "first", &mem0);

    let mut mem1 = aligned_alloc(size);
    print_info(false, "second", &mem1);

    size *= 4;
  }
}
