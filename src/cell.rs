use crate::dims::I3;
use std::mem;
use std::mem::MaybeUninit;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct SpatialCell<T: Clone> {
    position: I3,
    pub value: T,
}

impl<T: Clone> SpatialCell<T> {
    #[inline]
    pub(crate) fn new(position: I3, value: T) -> Self {
        Self { position, value }
    }

    #[inline]
    pub(crate) fn new_empty() -> Self {
        Self {
            position: [i32::MIN; 3].into(),
            value: unsafe { MaybeUninit::uninit().assume_init() }, // SAFETY: we use position.x as Some(T) discriminant
        }
    }

    #[inline]
    pub(crate) fn is_some(&self) -> bool {
        self.position[0] != i32::MIN
    }

    #[inline]
    pub(crate) fn pos_eq(&self, pos: impl Into<I3>) -> bool {
        let pos = pos.into();
        self.position[0] == pos[0] && self.position[1] == pos[1] && self.position[2] == pos[2]
    }

    #[inline]
    pub(crate) fn take(&mut self) -> Option<Self> {
        if self.is_some() {
            Some(mem::replace(self, Self::new_empty()))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn position(&self) -> &[i32; 3] {
        &self.position
    }
}
