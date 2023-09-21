use bumpalo::Bump;
use bumpalo::{collections::Vec as VecBump, boxed::Box as BoxBump};

#[derive(Debug)]
#[repr(transparent)]
pub struct Vec<'bump, T>(pub (crate)VecBump<'bump, T>);
#[derive(Debug)]
#[repr(transparent)]
pub struct Box<'bump, T>(pub (crate) BoxBump<'bump, T>);

impl<'bump, T> Vec<'bump, T> {
    #[inline]
    pub fn new_in(alloc: &'bump Bump) -> Self {
        Vec(VecBump::new_in(alloc))
    }
}

impl<'bump, T> Box<'bump, T> {
    #[inline]
    pub fn new_in(alloc: &'bump Bump, value: T) -> Self {
        Box(BoxBump::new_in(value, alloc))
    }
}

use serde::Serialize;
use serde::Serializer;

impl<T: Serialize> Serialize for Vec<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.collect_seq(self.0.iter())
    }
}

impl<T: Serialize> Serialize for Box<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        self.0.serialize(serializer)
    }
}