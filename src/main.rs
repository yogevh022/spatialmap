use std::hint::black_box;
use spatialmap::SpatialMap;

#[inline(always)]
pub const fn old(n: i32, p: i32) -> i32 {
    ((n % p) + p) % p
}

#[inline(always)]
pub const fn old_p2(n: i32, p: i32) -> i32 {
    n & (p - 1)
}

#[inline(always)]
pub const fn new(n: i32, p: i32) -> i32 {
    n.rem_euclid(p)
}

pub fn test_index() {
    const COUNT: usize = 100_000;
    let cap = black_box(64);
    let mut map = SpatialMap::<u64>::with_capacity([cap, cap, cap]);

    let time = std::time::Instant::now();
    for _ in 0..COUNT {
        for x in 0..cap as i32 {
            for y in 0..cap as i32 {
                for z in 0..cap as i32 {
                    let pos = [black_box(x), black_box(y), black_box(z)];
                    // let q = map.index(pos);
                    // black_box(q);
                }
            }
        }
    }
    let t = time.elapsed();
    println!("time: {:?}", t);
}

pub fn temp() {
    const COUNT: usize = 1_000_000;
    let cap = black_box(16);

    let time = std::time::Instant::now();
    for i in 0..COUNT as i32 {
        let i = black_box(i);
        black_box(old(i, cap));
    }
    let old_t = time.elapsed();

    let time = std::time::Instant::now();
    for i in 0..COUNT as i32 {
        let i = black_box(i);
        black_box(old_p2(i, cap));
    }
    let oldp2_t = time.elapsed();

    let time = std::time::Instant::now();
    for i in 0..COUNT as i32 {
        let i = black_box(i);
        black_box(new(i, cap));
    }
    let new_t = time.elapsed();

    println!("old: {:?}", old_t);
    println!("oldp2: {:?}", oldp2_t);
    println!("new: {:?}", new_t);
}

pub fn main() {
    test_index();
}