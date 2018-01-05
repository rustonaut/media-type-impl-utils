#![feature(test)]

extern crate test;

use test::{Bencher, black_box};


// is_ascii_alphanumeric_lowercase_letter ... bench:       1,828 ns/iter (+/- 594)
// is_alphanumeric_lowercase_letter       ... bench:       1,971 ns/iter (+/- 360)
//
// is_ascii_alphanumeric_uppercase_letter ... bench:       1,823 ns/iter (+/- 243)
// is_alphanumeric_uppercase_letter       ... bench:       1,992 ns/iter (+/- 141)
//
// is_ascii_alphanumeric_number           ... bench:       1,828 ns/iter (+/- 237)
// is_alphanumeric_number                 ... bench:       2,354 ns/iter (+/- 280)
//
// is_ascii_alphanumeric_tab              ... bench:       1,822 ns/iter (+/- 123)
// is_alphanumeric_tab                    ... bench:       2,383 ns/iter (+/- 125)
//
// is_ascii_alphanumeric_utf8             ... bench:       1,258 ns/iter (+/- 715)
// is_alphanumeric_utf8                   ... bench:       1,375 ns/iter (+/- 147)
//
// ====> is_ascii_alphanumeric is ignorable faster for ascii letters and utf8 and
//       slightly faster for numbers and tab (general non alphanumeric ascii?),
//       but not in a degree that it makes sense to add a "rust>=1.24" feature with
//       a cfg feature switch using ascii_... if the user wants to

#[bench]
fn is_ascii_alphanumeric_lowercase_letter(b: &mut Bencher) {
    let v = 'e' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v.is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric_lowercase_letter(b: &mut Bencher) {
    let v = 'e' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v < 0x7f && (v as char).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}


#[bench]
fn is_ascii_alphanumeric_uppercase_letter(b: &mut Bencher) {
    let v = 'E' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v.is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric_uppercase_letter(b: &mut Bencher) {
    let v = 'E' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v < 0x7f && (v as char).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}


#[bench]
fn is_ascii_alphanumeric_number(b: &mut Bencher) {
    let v = '3' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v.is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric_number(b: &mut Bencher) {
    let v = '3' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v < 0x7f && (v as char).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}

#[bench]
fn is_ascii_alphanumeric_tab(b: &mut Bencher) {
    let v = '\t' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v.is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric_tab(b: &mut Bencher) {
    let v = '\t' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v < 0x7f && (v as char).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}


#[bench]
fn is_ascii_alphanumeric_utf8(b: &mut Bencher) {
    let v = 'ä' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v.is_ascii_alphanumeric() { x += 1;} else { x -= 1;}
        }
        x
    })
}

#[bench]
fn is_alphanumeric_utf8(b: &mut Bencher) {
    let v = 'ä' as u8;
    b.iter(|| {
        let mut x = 0u64;
        for _ in 0..1000 {
            let v = black_box(v);
            if v < 0x7f && (v as char).is_alphanumeric() { x += 1; } else { x-=1; }
        }
        x
    })
}