use std::mem;
mod cell;
mod dims;
use cell::SpatialCell;
use dims::{I3, U3};

pub struct SpatialMap<T: Default + Clone> {
    data: Box<[SpatialCell<T>]>,
    dim: [i32; 3],
}

impl<T: Default + Clone> SpatialMap<T> {
    pub fn with_capacity(dim: impl Into<U3>) -> Self {
        let dim = dim.into();
        let len = dim.iter().product::<u32>() as usize;
        debug_assert!(len > 0);
        Self {
            data: vec![SpatialCell::new_empty(); len].into_boxed_slice(),
            dim: dim.map(|d| d as i32),
        }
    }

    #[inline]
    pub fn index(&self, position: impl Into<I3>) -> usize {
        let position = position.into();
        let (dim_x, dim_y, dim_z) = (self.dim[0], self.dim[1], self.dim[2]);
        let x = rem_e(position[0], dim_x);
        let y = rem_e(position[1], dim_y);
        let z = rem_e(position[2], dim_z);
        ((x * dim_y + y) * dim_z + z) as usize
    }

    pub fn insert(&mut self, position: impl Into<I3>, value: T) -> Option<SpatialCell<T>> {
        let position = position.into();
        let index = self.index(position);
        self.insert_index(index, position, value)
    }

    pub fn insert_index(
        &mut self,
        index: usize,
        position: impl Into<I3>,
        value: T,
    ) -> Option<SpatialCell<T>> {
        let mut swap_cell = SpatialCell::new(position.into(), value);
        mem::swap(&mut swap_cell, &mut self.data[index]);
        swap_cell.is_some().then_some(swap_cell)
    }

    pub fn get(&self, position: impl Into<I3>) -> Option<&SpatialCell<T>> {
        let index = self.index(position.into());
        self.get_index(index)
    }

    pub fn get_mut(&mut self, position: impl Into<I3>) -> Option<&mut SpatialCell<T>> {
        let index = self.index(position.into());
        self.get_index_mut(index)
    }

    pub fn get_index(&self, index: usize) -> Option<&SpatialCell<T>> {
        let cell = &self.data[index];
        cell.is_some().then_some(cell)
    }

    pub fn get_index_mut(&mut self, index: usize) -> Option<&mut SpatialCell<T>> {
        let cell = &mut self.data[index];
        cell.is_some().then_some(cell)
    }

    pub unsafe fn get_index_mut_unchecked(&mut self, index: usize) -> &mut SpatialCell<T> {
        &mut self.data[index]
    }

    pub fn get_exact(&self, position: impl Into<I3>) -> Option<&SpatialCell<T>> {
        let position = position.into();
        let index = self.index(position);
        let cell = &self.data[index];
        cell.pos_eq(position).then_some(cell)
    }

    pub fn get_exact_mut(&mut self, position: impl Into<I3>) -> Option<&mut SpatialCell<T>> {
        let position = position.into();
        let index = self.index(position);
        let cell = &mut self.data[index];
        cell.pos_eq(position).then_some(cell)
    }

    pub fn remove(&mut self, position: impl Into<I3>) -> Option<SpatialCell<T>> {
        let index = self.index(position.into());
        self.data[index].take()
    }

    pub fn remove_index(&mut self, index: usize) -> Option<SpatialCell<T>> {
        self.data[index].take()
    }

    pub fn remove_exact(&mut self, position: impl Into<I3>) -> Option<SpatialCell<T>> {
        let position = position.into();
        let index = self.index(position);
        let cell = &mut self.data[index];
        if cell.pos_eq(position) {
            cell.take()
        } else {
            None
        }
    }
}

#[inline(always)]
const fn rem_e(n: i32, p: i32) -> i32 {
    n.rem_euclid(p)
}

#[inline(always)]
const fn rem_e_p2(n: i32, p: i32) -> i32 {
    n & (p - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use std::time::Duration;

    fn print_bench_time(label: &'static str, dim: [u32; 3], count: usize, time: Duration) {
        let total = count * (dim[0] * dim[1] * dim[2]) as usize;
        println!("{:<16} {:?} - {} ({}*[{}*{}*{}])", label, time, total, count, dim[0], dim[1], dim[2]);
    }

    #[test]
    pub fn bench() {
        let count = black_box(1_000);
        let cap = black_box(64i32);
        let map_dim = black_box([cap as u32, cap as u32, cap as u32]);
        let mut map = SpatialMap::<u64>::with_capacity(map_dim);

        let time = std::time::Instant::now();
        for _ in 0..count {
            for x in 0..cap {
                for y in 0..cap {
                    for z in 0..cap {
                        let pos = black_box([x, y, z]);
                        let q = map.insert(pos, 0);
                        black_box(q);
                    }
                }
            }
        }
        print_bench_time("INSERT", map_dim, count, time.elapsed())
    }

    #[cfg(feature = "glam")]
    #[test]
    pub fn glam_bench() {
        use glam::{IVec3, UVec3};
        let count = black_box(1_000);
        let cap = black_box(64i32);
        let map_dim = black_box([cap as u32, cap as u32, cap as u32]);
        let mut map = SpatialMap::<u64>::with_capacity(map_dim);

        let time = std::time::Instant::now();
        for _ in 0..count {
            for x in 0..cap {
                for y in 0..cap {
                    for z in 0..cap {
                        let pos = black_box(IVec3::new(x, y, z));
                        let q = map.insert(pos, 0);
                        black_box(q);
                    }
                }
            }
        }
        print_bench_time("INSERT --glam", map_dim, count, time.elapsed())
    }
}
