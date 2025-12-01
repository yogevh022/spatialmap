use std::mem;

pub struct SpatialMap<T: Clone> {
    old_data: Box<[Option<([i32; 3], T)>]>,
    data: Box<[([i32; 3], T)]>,
    occupancy: Box<[bool]>,
    dim: [i32; 3],
}

impl<T: Clone> SpatialMap<T> {
    pub fn with_capacity(dim: [u32; 3]) -> Self {
        assert!(dim.iter().all(|d| d.is_power_of_two()));
        let len = dim.iter().product::<u32>() as usize;
        let data = unsafe {
            // SAFETY: we track init with the occupancy array
            Box::new_uninit_slice(len).assume_init()
        };
        let occupancy = vec![false; len].into_boxed_slice();
        Self {
            old_data: vec![None; len].into_boxed_slice(),
            data,
            occupancy,
            dim: dim.map(|d| d as i32),
        }
    }

    pub fn insert(&mut self, position: [i32; 3], value: T) -> Option<([i32; 3], T)> {
        let index = self.index(position);
        self.insert_index(index, position, value)
    }

    pub fn insert_index(
        &mut self,
        index: usize,
        position: [i32; 3],
        value: T,
    ) -> Option<([i32; 3], T)> {
        let mut swap_cell = (position, value);
        mem::swap(&mut swap_cell, &mut self.data[index]);
        if self.occupancy[index] {
            Some(swap_cell)
        } else {
            self.occupancy[index] = true;
            None
        }
    }

    pub fn get(&self, position: [i32; 3]) -> Option<&([i32; 3], T)> {
        let index = self.index(position);
        self.get_index(index)
    }

    pub fn get_exact(&self, position: [i32; 3]) -> Option<&T> {
        let index = self.index(position);
        let (cell_pos, value) = &self.data[index];
        if pos_eq(*cell_pos, position) && self.occupancy[index] {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&([i32; 3], T)> {
        if self.occupancy[index] {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn remove(&mut self, position: [i32; 3]) -> Option<([i32; 3], T)> {
        let index = self.index(position);
        if self.occupancy[index] {
            self.occupancy[index] = false;
            Some(self.data[index].clone())
        } else {
            None
        }
    }

    pub fn old_insert(&mut self, position: [i32; 3], value: T) -> Option<([i32; 3], T)> {
        let index = self.index(position);
        self.old_data[index].replace((position, value))
    }

    pub fn old_insert_index(
        &mut self,
        index: usize,
        position: [i32; 3],
        value: T,
    ) -> Option<([i32; 3], T)> {
        self.old_data[index].replace((position, value))
    }

    pub fn old_get(&self, position: [i32; 3]) -> Option<([i32; 3], &T)> {
        let index = self.index(position);
        self.old_data[index]
            .as_ref()
            .map(|(pos, val)| (pos.clone(), val))
    }

    pub fn old_get_exact(&self, position: [i32; 3]) -> Option<&T> {
        let index = self.index(position);
        if let Some((cell_pos, ref value)) = self.old_data[index] {
            if pos_eq(cell_pos, position) {
                return Some(value);
            }
        }
        None
    }

    pub fn old_remove(&mut self, position: [i32; 3]) -> Option<([i32; 3], T)> {
        let index = self.index(position);
        self.old_data[index].take()
    }

    pub fn old_remove_exact(&mut self, position: [i32; 3]) -> Option<T> {
        let index = self.index(position);
        if let Some((cell_pos, _)) = self.old_data[index] {
            if pos_eq(cell_pos, position) {
                return self.old_data[index].take().map(|(_, v)| v);
            }
        }
        None
    }





    // pub fn get_mut(&mut self, position: [i32; 3]) -> Option<([i32; 3], &mut T)> {
    //     let index = self.index(position);
    //     self.data[index].as_mut().map(|(pos, val)| (pos.clone(), val))
    // }
    //
    // pub fn get_mut_exact(&mut self, position: [i32; 3]) -> Option<&mut T> {
    //     let index = self.index(position);
    //     if let Some((cell_pos, _)) = self.data[index] {
    //         if pos_eq(cell_pos, position) {
    //             return self.data[index].as_mut().map(|(_, v)| v);
    //         }
    //     }
    //     None
    // }

    // pub fn get_index(&self, index: usize) -> Option<([i32; 3], &T)> {
    //     self.data[index].as_ref().map(|(pos, val)| (pos.clone(), val))
    // }
    //
    // pub fn get_index_exact(&self, index: usize, position: [i32; 3]) -> Option<&T> {
    //     if let Some((cell_pos, ref value)) = self.data[index] {
    //         if pos_eq(cell_pos, position) {
    //             return Some(value);
    //         }
    //     }
    //     None
    // }

    // pub fn get_index_mut(&mut self, index: usize) -> Option<([i32; 3], &mut T)> {
    //     self.data[index].as_mut().map(|(pos, val)| (pos.clone(), val))
    // }
    //
    // pub fn get_index_mut_exact(&mut self, index: usize, position: [i32; 3]) -> Option<&mut T> {
    //     if let Some((cell_pos, _)) = self.data[index] {
    //         if pos_eq(cell_pos, position) {
    //             return self.data[index].as_mut().map(|(_, v)| v);
    //         }
    //     }
    //     None
    // }

    #[inline(always)]
    pub fn index(&self, position: [i32; 3]) -> usize {
        let x = rem_e_p2(position[0], self.dim[0]);
        let y = rem_e_p2(position[1], self.dim[1]);
        let z = rem_e_p2(position[2], self.dim[2]);
        unsafe {
            // SAFETY: we validated the indices above (mod with max dim)
            self.index_unchecked(x, y, z)
        }
    }

    #[inline(always)]
    pub unsafe fn index_unchecked(&self, x: i32, y: i32, z: i32) -> usize {
        ((x * self.dim[1] + y) * self.dim[2] + z) as usize
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
const fn rem_e_mod(n: i32, p: i32) -> i32 {
    ((n % p) + p) % p
}

#[inline(always)]
const fn rem_e_p2(n: i32, p: i32) -> i32 {
    n & (p - 1)
}

#[inline(always)]
const fn rem_e(n: i32, p: i32) -> i32 {
    n.rem_euclid(p)
}

#[inline(always)]
const fn pos_eq(a: [i32; 3], b: [i32; 3]) -> bool {
    a[0] == b[0] && a[1] == b[1] && a[2] == b[2]
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
                        let q = map.old_insert(pos, 0);
                        black_box(q);
                    }
                }
            }
        }
        let old_t = time.elapsed();
        println!("old ins: {:?}", old_t);

        let time = std::time::Instant::now();
        for _ in 0..COUNT {
            for x in 0..cap as i32 {
                for y in 0..cap as i32 {
                    for z in 0..cap as i32 {
                        let pos = [black_box(x), black_box(y), black_box(z)];
                        let q = map.old_get(pos);
                        black_box(q);
                    }
                }
            }
        }
        let old_t = time.elapsed();
        println!("old get: {:?}", old_t);

        let time = std::time::Instant::now();
        for _ in 0..COUNT {
            for x in 0..cap as i32 {
                for y in 0..cap as i32 {
                    for z in 0..cap as i32 {
                        let pos = [black_box(x), black_box(y), black_box(z)];
                        let q = map.old_get_exact(pos);
                        black_box(q);
                    }
                }
            }
        }
        let old_t = time.elapsed();
        println!("old get exacat: {:?}", old_t);

        let time = std::time::Instant::now();
        for _ in 0..COUNT {
            for x in 0..cap as i32 {
                for y in 0..cap as i32 {
                    for z in 0..cap as i32 {
                        let pos = [black_box(x), black_box(y), black_box(z)];
                        let q = map.old_remove(pos);
                        black_box(q);
                    }
                }
            }
        }
        let old_t = time.elapsed();
        println!("old remove: {:?}", old_t);

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
        let new_t = time.elapsed();
        println!("new ins: {:?}", new_t);

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
        let new_t = time.elapsed();
        println!("new get: {:?}", new_t);

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
        let new_t = time.elapsed();
        println!("new get exact: {:?}", new_t);

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
        let new_t = time.elapsed();
        println!("new remove: {:?}", new_t);
    }
}
