use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Html {
    Form(Form),
    Div(Div),
    Input(Input),
}

impl Display for Html {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Html::Form(form) => write!(f, "{}", form),
            Html::Div(div) => write!(f, "{}", div),
            Html::Input(input) => write!(f, "{}", input),
        }
    }
}

impl From<Form> for Html {
    fn from(form: Form) -> Self {
        Html::Form(form)
    }
}

impl From<Div> for Html {
    fn from(div: Div) -> Self {
        Html::Div(div)
    }
}

impl From<Input> for Html {
    fn from(input: Input) -> Self {
        Html::Input(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Form {
    children: Vec<Html>,
}

impl Form {
    pub const fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_input(mut self, input: Input) -> Self {
        self.children.push(input.into());
        self
    }
}

impl Extend<Html> for Form {
    fn extend<T: IntoIterator<Item = Html>>(&mut self, iter: T) {
        self.children.extend(iter);
    }
}

impl Display for Form {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<form>{}</form>",
            self.children
                .iter()
                .map(Html::to_string)
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl From<Input> for Form {
    fn from(input: Input) -> Self {
        Form::new().with_input(input)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Div {
    children: Vec<Html>,
}

impl Div {
    pub const fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: Html) -> Self {
        self.children.push(child);
        self
    }
}

impl Display for Div {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<div>{}</div>",
            self.children
                .iter()
                .map(Html::to_string)
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Input {
    name: String,
    input_type: InputType,
    value: String,
    disabled: bool,
    readonly: bool,
    required: bool,
}

impl Input {
    pub const fn new(name: String) -> Self {
        Self {
            name,
            input_type: InputType::new(),
            value: String::new(),
            disabled: false,
            readonly: false,
            required: false,
        }
    }

    pub const fn with_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    pub const fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<input name=\"{}\" type=\"{}\" value=\"{}\"/>",
            self.name, self.input_type, self.value
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

impl InputType {
    pub const fn new() -> Self {
        Self::Text
    }
}

impl Display for InputType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Button => write!(f, "button"),
            InputType::Checkbox => write!(f, "checkbox"),
            InputType::Color => write!(f, "color"),
            InputType::Date => write!(f, "date"),
            InputType::DatetimeLocal => write!(f, "datetime-local"),
            InputType::Email => write!(f, "email"),
            InputType::File => write!(f, "file"),
            InputType::Hidden => write!(f, "hidden"),
            InputType::Image => write!(f, "image"),
            InputType::Month => write!(f, "month"),
            InputType::Number => write!(f, "number"),
            InputType::Password => write!(f, "password"),
            InputType::Radio => write!(f, "radio"),
            InputType::Range => write!(f, "range"),
            InputType::Reset => write!(f, "reset"),
            InputType::Search => write!(f, "search"),
            InputType::Submit => write!(f, "submit"),
            InputType::Tel => write!(f, "tel"),
            InputType::Text => write!(f, "text"),
            InputType::Time => write!(f, "time"),
            InputType::Url => write!(f, "url"),
            InputType::Week => write!(f, "week"),
        }
    }
}

impl Default for InputType {
    fn default() -> Self {
        InputType::new()
    }
}
