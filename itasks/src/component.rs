pub use itasks_derive::Component;

use crate::frontend::{Form, Input, InputValue};

pub trait Component {
    fn view(&self) -> Form;

    fn enter() -> Form;

    fn update(&self) -> Form;
}

impl Component for bool {
    fn view(&self) -> Form {
        self.update().readonly()
    }

    fn enter() -> Form {
        Self::default().update()
    }

    fn update(&self) -> Form {
        Form::new(vec![Input::new(InputValue::Truth(*self))])
    }
}

macro_rules! impl_num {
    ($($t:ty),*) => {
        $(
            impl Component for $t {
                fn view(&self) -> Form {
                    self.update().readonly()
                }

                fn enter() -> Form {
                    Self::default().update()
                }

                fn update(&self) -> Form {
                    Form::new(vec![Input::new(InputValue::Number(*self as i32))])
                }
            }
        )*
    };
}

impl_num!(isize, usize, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);

impl Component for char {
    fn view(&self) -> Form {
        self.update().readonly()
    }

    fn enter() -> Form {
        Self::default().update()
    }

    fn update(&self) -> Form {
        Form::new(vec![Input::new(InputValue::Character(*self))])
    }
}

impl Component for &str {
    fn view(&self) -> Form {
        self.update().readonly()
    }

    fn enter() -> Form {
        Self::default().update()
    }

    fn update(&self) -> Form {
        Form::new(vec![Input::new(InputValue::Text(self.to_string()))])
    }
}

impl Component for String {
    fn view(&self) -> Form {
        self.update().readonly()
    }

    fn enter() -> Form {
        Self::default().update()
    }

    fn update(&self) -> Form {
        Form::new(vec![Input::new(InputValue::Text(self.clone()))])
    }
}

impl Component for () {
    fn view(&self) -> Form {
        let inputs = vec![];
        Form::new(inputs).readonly()
    }

    fn enter() -> Form {
        let inputs = vec![];
        Form::new(inputs)
    }

    fn update(&self) -> Form {
        let inputs = vec![];
        Form::new(inputs)
    }
}

impl<C1> Component for (C1,)
where
    C1: Component,
{
    fn view(&self) -> Form {
        let inputs = vec![Input::new(InputValue::Form(self.0.view()))];
        Form::new(inputs).readonly()
    }

    fn enter() -> Form {
        let inputs = vec![Input::new(InputValue::Form(C1::enter()))];
        Form::new(inputs)
    }

    fn update(&self) -> Form {
        let inputs = vec![Input::new(InputValue::Form(self.0.update()))];
        Form::new(inputs)
    }
}

impl<C1, C2> Component for (C1, C2)
where
    C1: Component,
    C2: Component,
{
    fn view(&self) -> Form {
        let inputs = vec![
            Input::new(InputValue::Form(self.0.view())),
            Input::new(InputValue::Form(self.1.view())),
        ];
        Form::new(inputs).readonly()
    }

    fn enter() -> Form {
        let inputs = vec![
            Input::new(InputValue::Form(C1::enter())),
            Input::new(InputValue::Form(C2::enter())),
        ];
        Form::new(inputs)
    }

    fn update(&self) -> Form {
        let inputs = vec![
            Input::new(InputValue::Form(self.0.update())),
            Input::new(InputValue::Form(self.1.update())),
        ];
        Form::new(inputs)
    }
}
