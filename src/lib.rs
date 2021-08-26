use std::ops::Deref;

pub use itasks_derive;

use crate::component::Component;

pub mod component;
mod rocket;

pub struct Task<C> {
    inner: C,
}

pub fn view<C: Component>(inner: C) -> Task<C> {
    Task { inner }
}

impl<C> Deref for Task<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
