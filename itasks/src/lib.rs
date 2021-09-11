pub mod component;
mod html;
mod rocket;
pub mod task;

pub mod prelude {
    pub use crate::component::Component;
    pub use crate::task::{Task, enter, update, view};
}
