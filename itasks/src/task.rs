use std::collections::BTreeMap;
use std::marker::PhantomData;

use crate::component::Component;
use crate::frontend::{Form, Input, InputValue};

pub struct Task<C> {
    pub form: Form,
    phantom: PhantomData<C>,
}

impl<C: Component> Task<C> {
    fn new(form: Form) -> Task<C> {
        Task {
            form,
            phantom: PhantomData,
        }
    }

    pub fn and<D: Component>(self, other: Task<D>) -> Task<(C, D)> {
        Task::new(
            Form::new()
                .with_input(Input::new(InputValue::Form(self.form)))
                .with_input(Input::new(InputValue::Form(other.form))),
        )
    }

    pub fn actions<D>(self) -> Actions<C, D> {
        Actions {
            task: self,
            actions: BTreeMap::new(),
            phantom: Default::default(),
        }
    }
}

pub struct Actions<C, D> {
    task: Task<C>,
    actions: BTreeMap<Action, Box<dyn FnOnce(C) -> Task<D>>>,
    phantom: PhantomData<D>,
}

impl<C, D> Actions<C, D> {
    pub fn on(mut self, action: Action, f: impl FnOnce(C) -> Task<D> + 'static) -> Self {
        self.actions.insert(action, Box::new(f));
        self
    }

    pub fn finalize(self) -> Task<D> {
        Task {
            form: self
                .task
                .form
                .with_actions(self.actions.into_iter().map(|(k, _)| k).collect()),
            phantom: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Action {
    Ok,
    Cancel,
    Custom(String),
}

impl Action {
    pub fn label(&self) -> String {
        match self {
            Action::Ok => "Ok".to_owned(),
            Action::Cancel => "Cancel".to_owned(),
            Action::Custom(s) => s.clone(),
        }
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
