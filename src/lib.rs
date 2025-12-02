use std::mem;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct SpatialCell<T: Default + Clone> {
    position: [i32; 3],
    value: T,
}

impl<T: Default + Clone> SpatialCell<T> {
    #[inline]
    fn new(position: [i32; 3], value: T) -> Self {
        Self { position, value }
    }

    #[inline]
    fn new_empty() -> Self {
        Self {
            position: [i32::MIN; 3],
            value: Default::default(),
        }
    }

    #[inline]
    fn is_some(&self) -> bool {
        self.position[0] != i32::MIN
    }

    #[inline]
    fn pos_eq(&self, pos: [i32; 3]) -> bool {
        self.position[0] == pos[0] && self.position[1] == pos[1] && self.position[2] == pos[2]
    }

    #[inline]
    fn take(&mut self) -> Option<Self> {
        if self.is_some() {
            Some(mem::replace(self, Self::new_empty()))
        } else {
            None
        }
    }
}

pub struct SpatialMap<T: Default + Clone> {
    data: Box<[SpatialCell<T>]>,
    dim: [i32; 3],
}

impl<T: Default + Clone> SpatialMap<T> {
    pub fn with_capacity(dim: [u32; 3]) -> Self {
        assert!(dim.iter().all(|d| d.is_power_of_two()));
        let len = dim.iter().product::<u32>() as usize;
        Self {
            data: vec![SpatialCell::new_empty(); len].into_boxed_slice(),
            dim: dim.map(|d| d as i32),
        }
    }

    #[inline]
    pub fn index(&self, position: [i32; 3]) -> usize {
        let x = rem_e_p2(position[0], self.dim[0]);
        let y = rem_e_p2(position[1], self.dim[1]);
        let z = rem_e_p2(position[2], self.dim[2]);
        unsafe {
            // SAFETY: we validated the indices above (mod with max dim)
            self.index_unchecked(x, y, z)
        }
    }

    #[inline]
    pub unsafe fn index_unchecked(&self, x: i32, y: i32, z: i32) -> usize {
        ((x * self.dim[1] + y) * self.dim[2] + z) as usize
    }

    pub fn insert(&mut self, position: [i32; 3], value: T) -> Option<SpatialCell<T>> {
        let index = self.index(position);
        self.insert_index(index, position, value)
    }

    pub fn insert_index(
        &mut self,
        index: usize,
        position: [i32; 3],
        value: T,
    ) -> Option<SpatialCell<T>> {
        let mut swap_cell = SpatialCell::new(position, value);
        mem::swap(&mut swap_cell, &mut self.data[index]);
        swap_cell.is_some().then_some(swap_cell)
    }

    pub fn get(&self, position: [i32; 3]) -> Option<&SpatialCell<T>> {
        let index = self.index(position);
        self.get_index(index)
    }

    pub fn get_mut(&mut self, position: [i32; 3]) -> Option<&mut SpatialCell<T>> {
        let index = self.index(position);
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

    pub fn get_exact(&self, position: [i32; 3]) -> Option<&SpatialCell<T>> {
        let index = self.index(position);
        let cell = &self.data[index];
        cell.pos_eq(position).then_some(cell)
    }

    pub fn get_exact_mut(&mut self, position: [i32; 3]) -> Option<&mut SpatialCell<T>> {
        let index = self.index(position);
        let cell = &mut self.data[index];
        cell.pos_eq(position).then_some(cell)
    }

    pub fn remove(&mut self, position: [i32; 3]) -> Option<SpatialCell<T>> {
        let index = self.index(position);
        self.data[index].take()
    }

    pub fn remove_index(&mut self, index: usize) -> Option<SpatialCell<T>> {
        self.data[index].take()
    }

    pub fn remove_exact(&mut self, position: [i32; 3]) -> Option<SpatialCell<T>> {
        let index = self.index(position);
        let cell = &mut self.data[index];
        if cell.pos_eq(position) {
            cell.take()
        } else {
            None
        }
    }

    // #[inline(always)]
    // pub fn adj_indices(&self, position: [i32; 3]) -> [usize; 6] {
    //     let x = Self::mod_pow2(position[0], self.dim[0]);
    //     let y = Self::mod_pow2(position[1], self.dim[1]);
    //     let z = Self::mod_pow2(position[2], self.dim[2]);
    //
    //     let xp = Self::mod_pow2(position[0] + 1, self.dim[0]);
    //     let xm = Self::mod_pow2(position[0] - 1, self.dim[0]);
    //     let yp = Self::mod_pow2(position[1] + 1, self.dim[1]);
    //     let ym = Self::mod_pow2(position[1] - 1, self.dim[1]);
    //     let zp = Self::mod_pow2(position[2] + 1, self.dim[2]);
    //     let zm = Self::mod_pow2(position[2] - 1, self.dim[2]);
    //
    //     unsafe {
    //         // SAFETY: we validated the indices above (mod with max dim)
    //         [
    //             self.index_unchecked(xp, y, z),
    //             self.index_unchecked(xm, y, z),
    //             self.index_unchecked(x, yp, z),
    //             self.index_unchecked(x, ym, z),
    //             self.index_unchecked(x, y, zp),
    //             self.index_unchecked(x, y, zm),
    //         ]
    //     }
    // }
}

#[inline(always)]
const fn rem_e_p2(n: i32, p: i32) -> i32 {
    n & (p - 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;

    #[test]
    pub fn test_map_population() {
        const COUNT: usize = 1000;
        let cap = black_box(64);
        let mut map = SpatialMap::<u64>::with_capacity([cap, cap, cap]);
        let time = std::time::Instant::now();
        for _ in 0..COUNT {
            for x in 0..cap as i32 {
                for y in 0..cap as i32 {
                    for z in 0..cap as i32 {
                        let pos = [black_box(x), black_box(y), black_box(z)];
                        let q = map.insert(pos, 0);
                        black_box(q);
                    }
                }
            }
        }
        let newer_ins_t = time.elapsed();

        let time = std::time::Instant::now();
        for _ in 0..COUNT {
            for x in 0..cap as i32 {
                for y in 0..cap as i32 {
                    for z in 0..cap as i32 {
                        let pos = [black_box(x), black_box(y), black_box(z)];
                        let q = map.get(pos);
                        black_box(q);
                    }
                }
            }
        }
        let newer_get_t = time.elapsed();

        let time = std::time::Instant::now();
        for _ in 0..COUNT {
            for x in 0..cap as i32 {
                for y in 0..cap as i32 {
                    for z in 0..cap as i32 {
                        let pos = [black_box(x), black_box(y), black_box(z)];
                        let q = map.get_exact(pos);
                        black_box(q);
                    }
                }
            }
        }
        let newer_get_ex_t = time.elapsed();

        let time = std::time::Instant::now();
        for _ in 0..COUNT {
            for x in 0..cap as i32 {
                for y in 0..cap as i32 {
                    for z in 0..cap as i32 {
                        let pos = [black_box(x), black_box(y), black_box(z)];
                        let q = map.remove(pos);
                        black_box(q);
                    }
                }
            }
        }
        let newer_remove_t = time.elapsed();

        println!("INSERT:: {:?}", newer_ins_t);
        println!("GET:: {:?} ", newer_get_t);
        println!("GET_EXACT:: {:?}", newer_get_ex_t);
        println!("REMOVE:: {:?}", newer_remove_t);
    }
}
