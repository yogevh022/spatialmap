use std::hint::black_box;
use spatialmap::SpatialMap;
use std::arch::x86_64::*;

#[inline(never)]
pub fn arr_cmp(a: [i32; 3], b: [i32; 3]) -> bool {
    a[0] == b[0] && a[1] == b[1] && a[2] == b[2]
}

#[inline(never)]
pub fn arr_cmp_simd(a: [i32; 3], b: [i32; 3]) -> bool {
    unsafe {
        let a128 = _mm_setr_epi32(a[0], a[1], a[2], 0);
        let b128 = _mm_setr_epi32(b[0], b[1], b[2], 0);
        let cmp = _mm_cmpeq_epi32(a128, b128);
        let mask = _mm_movemask_epi8(cmp);
        mask & 0xFFF == 0xFFF
    }
}

pub fn test_index() {
    const COUNT: usize = 100_000;
    let cap = black_box(64);
    let mut map = SpatialMap::<u64>::with_capacity([cap, cap, cap]);

    black_box(arr_cmp([0, 0, 0], [0, 0, 0]));
    black_box(arr_cmp_simd([0, 0, 0], [0, 0, 0]));

    let time = std::time::Instant::now();
    for _ in 0..COUNT {
        for i in 0..(64*64) {
            let a = black_box([1, 2, 3]);
            let b = black_box([1, 2, 3]);
            let c = arr_cmp_simd(a, b);
            black_box(c);
        }
    }
    let t = time.elapsed();
    println!("time: {:?}", t);
}

pub fn main() {
    test_index();
}