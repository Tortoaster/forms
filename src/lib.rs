use crate::component::Component;
use std::ops::Deref;

mod component;
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
