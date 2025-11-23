pub struct SpatialMap<T: Copy + Clone> {
    data: Box<[Option<([i32; 3], T)>]>,
    dim: [i32; 3],
}

impl<T: Copy + Clone> SpatialMap<T> {
    pub fn with_capacity(dim: [u32; 3]) -> Self {
        assert!(dim.iter().all(|&d| d.is_power_of_two()));
        let len = dim.iter().product::<u32>() as usize;
        Self {
            data: vec![None; len].into_boxed_slice(),
            dim: dim.map(|d| d as i32),
        }
    }

    pub fn insert(&mut self, position: [i32; 3], value: T) -> Option<([i32; 3], T)> {
        let index = self.index(position);
        self.data[index].replace((position, value))
    }

    pub fn remove(&mut self, position: [i32; 3]) -> Option<([i32; 3], T)> {
        let index = self.index(position);
        self.data[index].take()
    }

    pub fn remove_exact(&mut self, position: [i32; 3]) -> Option<T> {
        let index = self.index(position);
        if let Some((cell_pos, _)) = self.data[index] {
            if Self::pos_equal(cell_pos, position) {
                return self.data[index].take().map(|(_, v)| v);
            }
        }
        None
    }

    pub fn get(&self, position: [i32; 3]) -> Option<([i32; 3], T)> {
        let index = self.index(position);
        self.data[index]
    }

    pub fn get_exact(&self, position: [i32; 3]) -> Option<&T> {
        let index = self.index(position);
        if let Some((cell_pos, ref value)) = self.data[index] {
            if Self::pos_equal(cell_pos, position) {
                return Some(value);
            }
        }
        None
    }

    #[inline(always)]
    pub fn index(&self, position: [i32; 3]) -> usize {
        let x = Self::wrap_mod(position[0], self.dim[0]);
        let y = Self::wrap_mod(position[1], self.dim[1]);
        let z = Self::wrap_mod(position[2], self.dim[2]);
        ((x * self.dim[1] + y) * self.dim[2] + z) as usize
    }

    #[inline(always)]
    fn wrap_mod(n: i32, m: i32) -> i32 {
        ((n % m) + m) % m
    }

    #[inline(always)]
    fn pos_equal(a: [i32; 3], b: [i32; 3]) -> bool {
        a[0] == b[0] && a[1] == b[1] && a[2] == b[2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_population() {
        let cap = 64;
        let mut map = SpatialMap::<u64>::with_capacity([cap, cap, cap]);

        let time = std::time::Instant::now();
        let mut i = 0;
        for _ in 0..1000 {
            for x in 0..cap {
                for y in 0..cap {
                    for z in 0..cap {
                        let pos = [x as i32, y as i32, z as i32];
                        map.insert(pos, i);
                        i += 1;
                    }
                }
            }
        }
        let elapsed = time.elapsed();

        println!("{:?}", elapsed);
    }
}
