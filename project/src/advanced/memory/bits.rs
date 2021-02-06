use super::helper::{count_bits, count_bits_val, count_ones, int_bits, print_bits};
use std::mem::{size_of, size_of_val};

// details of u8 impl
// https://doc.rust-lang.org/std/primitive.u8.html

struct Regular {
  field1: u8,
  field2: u16,
}

struct Regula2 {
  field1: u8,
  field2: u16,
  field3: u8,
}

struct Tuple(u8, u16);

struct Unit;
// Unit structs are most commonly used as marker. They have a size of zero bytes, but unlike empty enums they can be instantiated, making them isomorphic to the unit type (). Unit structs are useful when you need to implement a trait on something, but don't need to store any data inside it.

// ref: How many byte(s)/bits is U8 - char, U16...
// https://community.arm.com/developer/tools-software/tools/f/keil-forum/37529/how-many-byte-s-bits-is-u8---char-u16#:~:text=Sizeof(U8)%20is%201%20%2D,1%20byte%20(8%20bit).

pub fn test() {
  // let r = &Regular {
  //   field1: 0,
  //   field2: 9,
  // };
  // dbg!(size_of_val(r));
  dbg!(size_of::<Regular>()); // 4
  dbg!(size_of::<Regula2>()); // 4
  dbg!(size_of::<Tuple>()); // 4

  let n = 0b10101010u8;
  let m = 0b10111000u8;
  print_bits(count_bits_val(&n) as u8, n as usize); // 10101010
  print_bits(count_bits_val(&n) as u8, m.reverse_bits() as usize); // 00011101 // m.reverse_bits() != !m
}
