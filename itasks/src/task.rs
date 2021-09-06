use crate::component::Component;

pub enum Task<C> {
    View(C),
    Enter,
    Update(C),
    Either(Box<Task<C>>, Box<Task<C>>),
}

impl<C: Component> Task<C> {
    pub fn content(self) -> String {
        match self {
            Task::View(component) => component.view(),
            Task::Enter => C::enter(),
            Task::Update(mut component) => component.update(),
            Task::Either(left, right) => format!("<div class=\"component\"><div class=\"content\">{}{}</div></div>", left.content(), right.content())
        }
    }

    pub fn or(self, other: Task<C>) -> Task<C> {
        Task::Either(Box::new(self), Box::new(other))
    }
}

pub fn view<C: Component>(component: C) -> Task<C> {
    Task::View(component)
}

pub fn enter<C: Component>() -> Task<C> {
    Task::Enter
}

pub fn update<C: Component>(component: C) -> Task<C> {
    Task::Update(component)
}
