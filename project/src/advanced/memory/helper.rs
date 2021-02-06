// getconf LONG_BIT
pub fn count_bits(mut x: usize) -> u8 {
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
  count_bits(!0usize) // 64 bits
}

pub fn print_bits(x: usize) {
  let i = int_bits();
  for n in 1..=i {
    print!("{}", if (x >> n) & 1usize == 1 { "1" } else { "0" });
  }
  println!("");
}

pub fn test() {
  print_bits(!0usize);
}
