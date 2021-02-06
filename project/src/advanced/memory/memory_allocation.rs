use super::helper::{count_bits, count_bits_val, count_ones, int_bits, print_bits};

fn print_info(msg: &str, mem: &[u8]) {
  let addr = (&mem[0] as *const u8) as u64;
  let mut bound: u64 = 1;
  while addr & bound == 0 {
    bound <<= 1;
  }

  // show how to check the head of address when allocating a memory
  // head memory is like XXXXXXXXXXX1000
  // so compare until & op matches to 1(at some bit), which means not zero.
  print_bits(count_bits_val(&addr) as u8, addr as usize);
  print_bits(count_bits_val(&bound) as u8, bound as usize);
  print_bits(count_bits_val(&bound) as u8, (addr & bound) as usize);

  // bound はメモリブロックの先頭アドレスが、何バイト境界に乗っているかを示している
  println!(
    "{:>6} > size: {:>10} addr: 0x{:>012x} bound: {:>10}",
    msg,
    mem.len(),
    addr,
    bound
  );
}

pub fn test() {
  const ONEGB: usize = 1024 * 1024 * 1024;
  let mut size: usize = 2;
  while size <= ONEGB {
    let mut mem0: Vec<u8> = Vec::with_capacity(size);
    unsafe { mem0.set_len(size) }
    print_info("first", &mem0);

    let mut mem1: Vec<u8> = Vec::with_capacity(size);
    unsafe { mem1.set_len(size) }
    print_info("second", &mem1);

    size *= 2;
  }
}
