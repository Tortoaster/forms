use std::marker::PhantomData;

use crate::component::Component;
use crate::html::Html;

pub struct Task<C> {
    pub content: Html,
    phantom: PhantomData<C>,
}

impl<C: Component> Task<C> {
    fn new(content: Html) -> Task<C> {
        Task {
            content,
            phantom: PhantomData::default()
        }
    }

    pub fn and<D: Component>(self, other: Task<D>) -> Task<(C, D)> {
        Task::new(format!("<div>{}{}</div>", self.content, other.content))
    }
}

pub fn view<C: Component>(component: C) -> Task<C> {
    Task::new(component.view())
}

pub fn enter<C: Component>() -> Task<C> {
    Task::new(C::enter())
}

pub fn update<C: Component>(component: C) -> Task<C> {
    Task::new(component.update())
}
