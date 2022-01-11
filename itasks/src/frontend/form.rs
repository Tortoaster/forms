use std::iter::FromIterator;

pub struct Form {
    pub(crate) title: Option<String>,
    pub(crate) inputs: Vec<Input>,
    pub(in crate::frontend) readonly: bool,
}

impl Form {
    pub fn new(inputs: Vec<Input>) -> Self {
        Form {
            title: None,
            inputs,
            readonly: false,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
}

impl IntoIterator for Form {
    type Item = Input;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match &self.title {
            None => self.inputs.into_iter(),
            Some(_) => vec![Input::new(InputValue::Form(self))].into_iter(),
        }
    }
}

impl FromIterator<Input> for Form {
    fn from_iter<T: IntoIterator<Item = Input>>(iter: T) -> Self {
        let inputs: Vec<Input> = iter.into_iter().collect();
        Form::new(inputs)
    }
}

pub struct Input {
    pub(in crate::frontend) value: InputValue,
    pub(in crate::frontend) hint: String,
}

impl Input {
    pub fn new(value: InputValue) -> Self {
        Input {
            value,
            hint: String::new(),
        }
    }

    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint = hint;
        self
    }
}

pub enum InputValue {
    Form(Form),
    Text(String),
    Character(char),
    Number(i32),
    Truth(bool),
}
