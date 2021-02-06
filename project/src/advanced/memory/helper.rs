use std::any::Any;
use std::mem::{self, size_of, size_of_val};

// getconf LONG_BIT

pub fn count_ones(mut x: usize) -> u8 {
  let mut bits = 0u8;
  while x > 0 {
    if x & 1usize == 1 {
      bits += 1;
    }
    x >>= 1;
  }
  bits as u8
}

pub fn int_bits() -> u8 {
  count_ones(!0usize) // 64 bits
}

pub fn print_bits(bits_count: u8, x: usize) {
  // let bits_count = size_of_val(&x); // converted to actual size like 8 bits
  for n in (0..bits_count).rev() {
    print!("{}", if (x >> n) & 1usize == 1 { "1" } else { "0" });
  }
  println!("");
}

pub fn count_bits<T>() -> usize {
  size_of::<T>() * 8
}

pub fn count_bits_val<T: Any>(x: &T) -> usize {
  size_of_val(x) * 8
}

pub fn test() {
  print_bits(int_bits(), !0usize);
  println!("{}", count_bits::<u8>());
}
