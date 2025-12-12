#[cfg(feature = "glam")]
use glam::{IVec3, UVec3};

use std::ops::Deref;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct I3 {
    inner: [i32; 3],
}

impl Deref for I3 {
    type Target = [i32; 3];
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<[i32; 3]> for I3 {
    fn from(dim: [i32; 3]) -> Self {
        Self { inner: dim }
    }
}

impl From<(isize, isize, isize)> for I3 {
    fn from(dim: (isize, isize, isize)) -> Self {
        Self {
            inner: [dim.0 as i32, dim.1 as i32, dim.2 as i32],
        }
    }
}

#[cfg(feature = "glam")]
impl From<glam::IVec3> for I3 {
    fn from(v: glam::IVec3) -> Self {
        Self {
            inner: v.to_array(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct U3 {
    inner: [u32; 3],
}

impl Deref for U3 {
    type Target = [u32; 3];
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<[u32; 3]> for U3 {
    fn from(dim: [u32; 3]) -> Self {
        Self { inner: dim }
    }
}

#[cfg(feature = "glam")]
impl From<glam::UVec3> for U3 {
    fn from(v: glam::UVec3) -> Self {
        Self {
            inner: v.to_array(),
        }
    }
}
