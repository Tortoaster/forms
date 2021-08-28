use crate::component::Component;

pub enum Task<C> {
    View(C),
    Enter,
    Update(C),
}

impl<C: Component> Task<C> {
    pub fn content(self) -> String {
        match self {
            Task::View(component) => component.view(),
            Task::Enter => C::enter(),
            Task::Update(mut component) => component.update()
        }
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
