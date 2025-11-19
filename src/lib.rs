#[derive(Debug, Copy, Clone)]
enum SpatialCell<T: Copy + Clone> {
    Occupied([i32; 3], T),
    Empty,
}

pub struct SpatialMap<T: Copy + Clone> {
    data: Box<[SpatialCell<T>]>,
    dim: [i32; 3],
}

impl<T: Copy + Clone> SpatialMap<T> {
    pub fn with_capacity(dim: [u32; 3]) -> Self {
        assert!(dim.iter().all(|&d| d.is_power_of_two()));
        let len = dim.iter().product::<u32>() as usize;
        Self {
            data: vec![SpatialCell::Empty; len].into_boxed_slice(),
            dim: dim.map(|d| d as i32),
        }
    }

    pub fn insert(&mut self, position: [i32; 3], value: T) {
        let index = self.index(position);
        let i3 = unsafe { std::mem::transmute(position) };
        self.data[index] = SpatialCell::Occupied(i3, value);
    }

    pub fn remove(&mut self, position: [i32; 3]) {
        let index = self.index(position);
        self.data[index] = SpatialCell::Empty;
    }

    pub fn get(&self, position: [i32; 3]) -> Option<&T> {
        let index = self.index(position);
        match self.data[index] {
            SpatialCell::Occupied(cell_pos, ref value) if Self::pos_equal(cell_pos, position) => {
                Some(value)
            }
            _ => None,
        }
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