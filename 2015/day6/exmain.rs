#![feature(test)]
extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn bench_xor_1000_ints(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1000);

            (0..n).fold(0, |a, b| a^ b)
        });
    }
}

fn main() {
    println!("Hello, world!");
}
