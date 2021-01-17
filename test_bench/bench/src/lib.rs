#![feature(test)] // nightly only. so change rust to nightly: rustup default nightly
extern crate test;

pub fn hash(x: u64) -> u64 {
    let mut y = x;
    for _ in 0..512 {
        y = y << 5;
        y = y ^ 5;
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_hash(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(2);
            hash(n);
        });
    }
}
