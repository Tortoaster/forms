pub mod component;
mod rocket;
pub mod task;

pub mod prelude {
    pub use crate::component::Component;
    pub use crate::task::{Task, enter, update, view};
}
