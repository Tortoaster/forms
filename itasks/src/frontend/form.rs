use std::collections::BTreeMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Form {
    pub(crate) title: Option<String>,
    pub(crate) inputs: Vec<Input>,
    pub(in crate::frontend) readonly: bool,
}

impl Form {
    pub fn new() -> Self {
        Form {
            title: None,
            inputs: Vec::new(),
            readonly: false,
        }
    }

    pub fn with_input(mut self, input: Input) -> Self {
        self.inputs.push(input);
        self
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

#[derive(Debug)]
pub struct Input {
    pub(in crate::frontend) value: InputValue,
    pub(in crate::frontend) hint: Option<String>,
}

impl Input {
    pub fn new(value: InputValue) -> Self {
        Input { value, hint: None }
    }

    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint = Some(hint);
        self
    }
}

#[derive(Debug)]
pub enum InputValue {
    Form(Form),
    Text(String),
    Character(char),
    Number(i32),
    Truth(bool),
    Choice(BTreeMap<String, Form>, String),
}

impl IntoIterator for Form {
    type Item = Input;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inputs.into_iter()
        // if self.inputs.len() == 1 && self.title.is_none() {
        //     self.inputs.into_iter()
        // } else {
        //     vec![Input::new(InputValue::Form(self))].into_iter()
        // }
    }
}

impl FromIterator<Input> for Form {
    fn from_iter<T: IntoIterator<Item = Input>>(iter: T) -> Self {
        let inputs: Vec<Input> = iter.into_iter().collect();

        Form {
            title: None,
            inputs,
            readonly: false,
        }
    }
}

impl IntoIterator for Input {
    type Item = Input;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        if let InputValue::Form(mut form) = self.value {
            if form.inputs.len() == 1 && form.title.is_none() {
                match self.hint {
                    None => form.inputs.into_iter(),
                    Some(hint) => vec![form.inputs.remove(0).with_hint(hint)].into_iter(),
                }
            } else {
                vec![Input::new(InputValue::Form(form))].into_iter()
            }
        } else {
            vec![self].into_iter()
        }
    }
}
