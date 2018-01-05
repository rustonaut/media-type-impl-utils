#![feature(test)]

extern crate test;

use test::{Bencher, black_box};

#[bench]
fn is_ascii_alphanumeric(b: &mut Bencher) {
    let v = 'b';
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            if black_box(v).is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric(b: &mut Bencher) {
    let v = 'b';
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
           if black_box(v).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}