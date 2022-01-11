use crate::frontend::form::{Form, Input, InputValue};

impl Form {
    pub fn as_html(&self) -> String {
        format!(
            "<form>{}</form>",
            self.inputs
                .iter()
                .map(|input| input.as_html(self.readonly))
                .collect::<Vec<_>>()
                .join("<br/>")
        )
    }
}

impl Input {
    pub fn as_html(&self, readonly: bool) -> String {
        match &self.value {
            InputValue::Form(form) => form.as_html(),
            InputValue::Text(text) => format!(
                "<input placeholder=\"{}\" value=\"{}\" {}/>",
                self.hint,
                text,
                readonly.then(|| "readonly").unwrap_or_default()
            ),
            InputValue::Character(character) => format!(
                "<input placeholder=\"{}\" value=\"{}\" max_length=\"1\" {}/>",
                self.hint,
                character,
                readonly.then(|| "readonly").unwrap_or_default()
            ),
            InputValue::Number(number) => format!(
                "<input placeholder=\"{}\" type=\"number\" value=\"{}\" {}/>",
                self.hint,
                number,
                readonly.then(|| "readonly").unwrap_or_default()
            ),
            InputValue::Truth(truth) => format!(
                "<label>{}: <input type=\"checkbox\" {} {}/></label>",
                self.hint,
                truth.then(|| "checked").unwrap_or_default(),
                readonly.then(|| "readonly").unwrap_or_default()
            ),
        }
    }
}
