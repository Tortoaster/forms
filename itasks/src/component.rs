use crate::frontend::{Form, Input, Value};
pub use itasks_derive::Component;

pub trait Component {
    fn view(&self) -> Form;

    fn enter() -> Form;

    fn update(&self) -> Form;
}

impl Component for bool {
    fn view(&self) -> Form {
        Input::new(Value::Truth(*self)).readonly().into()
    }

    fn enter() -> Form {
        Input::new(Value::Truth(false)).into()
    }

    fn update(&self) -> Form {
        Input::new(Value::Truth(*self)).into()
    }
}

macro_rules! impl_num {
    ($($t:ty),*) => {
        $(
            impl Component for $t {
                fn view(&self) -> Form {
                    Input::new(Value::Number(*self as i32))
                        .readonly()
                        .into()
                }

                fn enter() -> Form {
                    Input::new(Value::Number(0))
                        .into()
                }

                fn update(&self) -> Form {
                    Input::new(Value::Number(*self as i32))
                        .into()
                }
            }
        )*
    };
}

impl_num!(isize, usize, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl Component for char {
    fn view(&self) -> Form {
        Input::new(Value::Character(*self)).readonly().into()
    }

    fn enter() -> Form {
        Input::new(Value::Character(' ')).into()
    }

    fn update(&self) -> Form {
        Input::new(Value::Character(*self)).into()
    }
}

impl Component for &str {
    fn view(&self) -> Form {
        Input::new(Value::Text(self.to_string())).readonly().into()
    }

    fn enter() -> Form {
        Input::new(Value::Text(String::new())).into()
    }

    fn update(&self) -> Form {
        Input::new(Value::Text(self.to_string())).into()
    }
}

impl Component for String {
    fn view(&self) -> Form {
        Input::new(Value::Text(self.clone())).readonly().into()
    }

    fn enter() -> Form {
        Input::new(Value::Text(String::new())).into()
    }

    fn update(&self) -> Form {
        Input::new(Value::Text(self.clone())).into()
    }
}

impl Component for () {
    fn view(&self) -> Form {
        Form::Compound(vec![])
    }

    fn enter() -> Form {
        Form::Compound(vec![])
    }

    fn update(&self) -> Form {
        Form::Compound(vec![])
    }
}

impl<C1> Component for (C1,)
where
    C1: Component,
{
    fn view(&self) -> Form {
        Form::Compound(vec![self.0.view()])
    }

    fn enter() -> Form {
        Form::Compound(vec![C1::enter()])
    }

    fn update(&self) -> Form {
        Form::Compound(vec![self.0.update()])
    }
}

impl<C1, C2> Component for (C1, C2)
where
    C1: Component,
    C2: Component,
{
    fn view(&self) -> Form {
        Form::Compound(vec![self.0.view(), self.1.view()])
    }

    fn enter() -> Form {
        Form::Compound(vec![C1::enter(), C2::enter()])
    }

    fn update(&self) -> Form {
        Form::Compound(vec![self.0.update(), self.1.update()])
    }
}
