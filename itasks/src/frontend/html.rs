use crate::frontend::form::{Form, Input, Value};

impl Form {
    pub fn as_html(&self) -> String {
        let fields = match self {
            Form::Unit(input, hint) => input.as_html(hint),
            Form::Compound(inputs) => inputs
                .iter()
                .map(Form::as_html)
                .collect::<Vec<_>>()
                .join("<br/>"),
        };

        format!("<form>{}</form>", fields)
    }
}

impl Input {
    pub fn as_html(&self, hint: &str) -> String {
        match &self.value {
            Value::Text(text) => format!(
                "<input placeholder=\"{}\" value=\"{}\" {}/>",
                hint,
                text,
                self.readonly.then(|| "readonly").unwrap_or_default()
            ),
            Value::Character(character) => format!(
                "<input placeholder=\"{}\" value=\"{}\" max_length=\"1\" {}/>",
                hint,
                character,
                self.readonly.then(|| "readonly").unwrap_or_default()
            ),
            Value::Number(number) => format!(
                "<input placeholder=\"{}\" type=\"number\" value=\"{}\" {}/>",
                hint,
                number,
                self.readonly.then(|| "readonly").unwrap_or_default()
            ),
            Value::Truth(truth) => format!(
                "<label>{}: <input type=\"checkbox\" {} {}/></label>",
                hint,
                truth.then(|| "checked").unwrap_or_default(),
                self.readonly.then(|| "readonly").unwrap_or_default()
            ),
        }
    }
}
