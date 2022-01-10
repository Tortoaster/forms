pub enum Form {
    Compound(Vec<Form>),
    Unit(Input, String),
}

impl Form {
    pub fn with_hint(self, hint: String) -> Self {
        match self {
            Form::Compound(forms) => Form::Compound(forms),
            Form::Unit(input, _) => Form::Unit(input, hint),
        }
    }
}

impl From<Input> for Form {
    fn from(input: Input) -> Self {
        Form::Unit(input, String::new())
    }
}

pub struct Input {
    pub(in crate::frontend) value: Value,
    pub(in crate::frontend) readonly: bool,
}

impl Input {
    pub fn new(value: Value) -> Self {
        Input {
            value,
            readonly: false,
        }
    }

    pub fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }
}

pub enum Value {
    Text(String),
    Character(char),
    Number(i32),
    Truth(bool),
}
