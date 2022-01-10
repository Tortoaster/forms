use std::marker::PhantomData;

use crate::component::Component;
use crate::html::{Div, Html};

pub struct Task<C> {
    pub content: Html,
    phantom: PhantomData<C>,
}

impl<C: Component> Task<C> {
    fn new(content: Html) -> Task<C> {
        Task {
            content,
            phantom: Default::default(),
        }
    }

    pub fn and<D: Component>(self, other: Task<D>) -> Task<(C, D)> {
        Task::new(Html::Div(Div::new().with_child(self.content).with_child(other.content)))
    }

    pub fn actions<D>(self) -> Actions<C, D> {
        Actions {
            task: self,
            phantom: Default::default(),
        }
    }
}

pub struct Actions<C, D> {
    task: Task<C>,
    phantom: PhantomData<D>,
}

impl<C, D> Actions<C, D> {
    pub fn on(self, _action: Action, _f: impl FnOnce(C) -> Task<D>) -> Self {
        self
    }

    pub fn finalize(self) -> Task<D> {
        Task {
            content: self.task.content,
            phantom: Default::default(),
        }
    }
}

pub enum Action {
    Ok,
    Cancel,
    Custom(String),
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
