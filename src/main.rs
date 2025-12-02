use std::hint::black_box;
use spatialmap::SpatialMap;

pub fn test_index() {
    const COUNT: usize = 1_000;
    let cap = black_box(64);
    let mut map = SpatialMap::<u64>::with_capacity([cap, cap, cap]);

    let time = std::time::Instant::now();
    for _ in 0..COUNT {
        for x in 0..cap as i32 {
            for y in 0..cap as i32 {
                for z in 0..cap as i32 {
                    let pos = [black_box(x), black_box(y), black_box(z)];
                    let q = map.index(pos);
                    black_box(q);
                }
            }
        }
    }
    let t = time.elapsed();
    println!("time: {:?}", t);
}

pub fn main() {
    test_index();
}