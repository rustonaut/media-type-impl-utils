#![feature(test)]

extern crate test;

use test::{Bencher, black_box};

// is_alphanumeric_lowercase_letter       ... bench:       1,810 ns/iter (+/- 151)
// is_alphanumeric_number                 ... bench:       2,387 ns/iter (+/- 102)
// is_alphanumeric_uppercase_letter       ... bench:       1,794 ns/iter (+/- 214)
// is_ascii_alphanumeric_lowercase_letter ... bench:       2,228 ns/iter (+/- 483)
// is_ascii_alphanumeric_number           ... bench:       2,199 ns/iter (+/- 155)
// is_ascii_alphanumeric_uppercase_letter ... bench:       2,210 ns/iter (+/- 316)
//
//====> woops alphanumeric is tendentially faster then ascii_alphanumeric this is, well strange?
//=====> but yes, the difference is in a very small time scall (one loop iteration is between
//       1.5-2.3ns without the overhead to prevent optimization probably something like 0.75-1.15ns
//       through howerver often I run this test alphanumeric on letter os always slower (somtimes
//       with more sometimes with less variance)

#[bench]
fn is_ascii_alphanumeric_lowercase_letter(b: &mut Bencher) {
    let v = 'e';
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            if black_box(v).is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric_lowercase_letter(b: &mut Bencher) {
    let v = 'e';
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
           if black_box(v).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}


#[bench]
fn is_ascii_alphanumeric_uppercase_letter(b: &mut Bencher) {
    let v = 'E';
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            if black_box(v).is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric_uppercase_letter(b: &mut Bencher) {
    let v = 'E';
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            if black_box(v).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}


#[bench]
fn is_ascii_alphanumeric_number(b: &mut Bencher) {
    let v = '3';
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            if black_box(v).is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric_number(b: &mut Bencher) {
    let v = '3';
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            if black_box(v).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}