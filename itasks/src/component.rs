pub use itasks_derive::Component;

use crate::frontend::html::{Div, Html, Input, InputType};

pub trait Component {
    fn view(&self) -> Html;

    fn enter() -> Html;

    fn update(&self) -> Html;
}

impl Component for bool {
    fn view(&self) -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Checkbox)
            .with_value(self.to_string())
            .readonly()
            .into()
    }

    fn enter() -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Checkbox)
            .into()
    }

    fn update(&self) -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Checkbox)
            .with_value(self.to_string())
            .into()
    }
}

macro_rules! impl_num {
    ($($t:ty),*) => {
        $(
            impl Component for $t {
                fn view(&self) -> Html {
                    Input::new("".to_owned())
                        .with_type(InputType::Number)
                        .with_value(self.to_string())
                        .readonly()
                        .into()
                }

                fn enter() -> Html {
                    Input::new("".to_owned())
                        .with_type(InputType::Number)
                        .into()
                }

                fn update(&self) -> Html {
                    Input::new("".to_owned())
                        .with_type(InputType::Number)
                        .with_value(self.to_string())
                        .into()
                }
            }
        )*
    };
}

impl_num!(isize, usize, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl Component for char {
    fn view(&self) -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Text)
            .with_value(self.to_string())
            .readonly()
            .into()
    }

    fn enter() -> Html {
        Input::new("".to_owned()).with_type(InputType::Text).into()
    }

    fn update(&self) -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Text)
            .with_value(self.to_string())
            .into()
    }
}

impl Component for &str {
    fn view(&self) -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Text)
            .with_value(self.to_string())
            .readonly()
            .into()
    }

    fn enter() -> Html {
        Input::new("".to_owned()).with_type(InputType::Text).into()
    }

    fn update(&self) -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Text)
            .with_value(self.to_string())
            .into()
    }
}

impl Component for String {
    fn view(&self) -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Text)
            .with_value(self.clone())
            .readonly()
            .into()
    }

    fn enter() -> Html {
        Input::new("".to_owned()).with_type(InputType::Text).into()
    }

    fn update(&self) -> Html {
        Input::new("".to_owned())
            .with_type(InputType::Text)
            .with_value(self.clone())
            .into()
    }
}

impl Component for () {
    fn view(&self) -> Html {
        Div::new().into()
    }

    fn enter() -> Html {
        Div::new().into()
    }

    fn update(&self) -> Html {
        Div::new().into()
    }
}

impl<C1> Component for (C1,)
where
    C1: Component,
{
    fn view(&self) -> Html {
        Div::new().with_child(self.0.view()).into()
    }

    fn enter() -> Html {
        Div::new().with_child(C1::enter()).into()
    }

    fn update(&self) -> Html {
        Div::new().with_child(self.0.update()).into()
    }
}

impl<C1, C2> Component for (C1, C2)
where
    C1: Component,
    C2: Component,
{
    fn view(&self) -> Html {
        Div::new()
            .with_child(self.0.view())
            .with_child(self.1.view())
            .into()
    }

    fn enter() -> Html {
        Div::new()
            .with_child(C1::enter())
            .with_child(C2::enter())
            .into()
    }

    fn update(&self) -> Html {
        Div::new()
            .with_child(self.0.update())
            .with_child(self.1.update())
            .into()
    }
}

impl<C1, C2, C3> Component for (C1, C2, C3)
where
    C1: Component,
    C2: Component,
    C3: Component,
{
    fn view(&self) -> Html {
        Div::new()
            .with_child(self.0.view())
            .with_child(self.1.view())
            .with_child(self.2.view())
            .into()
    }

    fn enter() -> Html {
        Div::new()
            .with_child(C1::enter())
            .with_child(C2::enter())
            .with_child(C3::enter())
            .into()
    }

    fn update(&self) -> Html {
        Div::new()
            .with_child(self.0.update())
            .with_child(self.1.update())
            .with_child(self.2.update())
            .into()
    }
}

impl<C1, C2, C3, C4> Component for (C1, C2, C3, C4)
where
    C1: Component,
    C2: Component,
    C3: Component,
    C4: Component,
{
    fn view(&self) -> Html {
        Div::new()
            .with_child(self.0.view())
            .with_child(self.1.view())
            .with_child(self.2.view())
            .with_child(self.3.view())
            .into()
    }

    fn enter() -> Html {
        Div::new()
            .with_child(C1::enter())
            .with_child(C2::enter())
            .with_child(C3::enter())
            .with_child(C4::enter())
            .into()
    }

    fn update(&self) -> Html {
        Div::new()
            .with_child(self.0.update())
            .with_child(self.1.update())
            .with_child(self.2.update())
            .with_child(self.3.update())
            .into()
    }
}
