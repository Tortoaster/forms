use std::fmt::Write;

use crate::frontend::form::{Form, Input, InputValue};

struct Context {
    form: usize,
    input: usize,
}

impl Context {
    fn new() -> Self {
        Self { form: 0, input: 0 }
    }

    fn new_form(&mut self) -> String {
        self.form += 1;
        self.input = 0;
        format!("form{}", self.form)
    }

    fn current_form(&self) -> String {
        format!("form{}", self.form)
    }

    fn new_input(&mut self) -> String {
        self.input += 1;
        format!("input{}", self.input)
    }
}

impl Form {
    pub fn html(&self) -> Result<String, std::fmt::Error> {
        let mut ctx = Context::new();
        self.as_html(&mut ctx)
    }

    fn as_html(&self, ctx: &mut Context) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        self.write_html(&mut s, ctx)?;
        Ok(s)
    }

    fn write_html(&self, s: &mut String, ctx: &mut Context) -> std::fmt::Result {
        write!(s, "<div class=\"component\">")?;

        if let Some(title) = &self.title {
            write!(s, "<div class=\"title\">{}</div>", title)?;
        }

        write!(s, "<form id=\"{}\" method=\"POST\"></form>", ctx.new_form())?;
        write!(s, "<div class=\"content\">")?;
        write!(
            s,
            "{}",
            self.inputs
                .iter()
                .map(|input| input.as_html(self.readonly, ctx))
                .collect::<Result<Vec<_>, _>>()?
                .join("<br/>")
        )?;
        write!(s, "<div class=\"actions\">")?;
        for action in &self.actions {
            let id = ctx.new_input();
            write!(
                s,
                "<input form=\"{}\" id=\"{id}\" name=\"{id}\" type=\"submit\" value=\"{}\"/>",
                ctx.current_form(),
                action.label()
            )?;
        }
        write!(s, "</div>")?;
        write!(s, "</div>")?;

        write!(s, "</div>")
    }
}

impl Input {
    fn as_html(&self, readonly: bool, ctx: &mut Context) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        self.write_html(&mut s, readonly, ctx)?;
        Ok(s)
    }

    fn write_html(&self, s: &mut String, readonly: bool, ctx: &mut Context) -> std::fmt::Result {
        let id = ctx.new_input();
        match &self.value {
            InputValue::Form(form) => {
                if let Some(hint) = &self.hint {
                    write!(s, "{}: ", hint)?;
                }
                write!(s, "{}", form.as_html(ctx)?)
            }
            InputValue::Text(text) => {
                write!(s, "<input")?;
                write!(s, " form=\"{}\"", ctx.current_form())?;
                write!(s, " id=\"{}\"", id)?;
                write!(s, " name=\"{}\"", id)?;
                write!(s, " type=\"text\"")?;
                write!(s, " value=\"{}\"", text)?;
                if readonly {
                    write!(s, " readonly")?;
                }
                write!(s, "/>")?;
                if let Some(label) = &self.hint {
                    write!(s, "<label for=\"{}\">{}</label>", id, label)?;
                }
                Ok(())
            }
            InputValue::Character(character) => {
                write!(s, "<input")?;
                write!(s, " form=\"{}\"", ctx.current_form())?;
                write!(s, " id=\"{}\"", id)?;
                write!(s, " name=\"{}\"", id)?;
                write!(s, " type=\"text\"")?;
                write!(s, " value=\"{}\"", character)?;
                write!(s, " max_length=\"1\"")?;
                if readonly {
                    write!(s, " readonly")?;
                }
                write!(s, "/>")?;
                if let Some(label) = &self.hint {
                    write!(s, "<label for=\"{}\">{}</label>", id, label)?;
                }
                Ok(())
            }
            InputValue::Number(number) => {
                write!(s, "<input")?;
                write!(s, " form=\"{}\"", ctx.current_form())?;
                write!(s, " id=\"{}\"", id)?;
                write!(s, " name=\"{}\"", id)?;
                write!(s, " type=\"number\"")?;
                write!(s, " value=\"{}\"", number)?;
                if readonly {
                    write!(s, " readonly")?;
                }
                write!(s, "/>")?;
                if let Some(label) = &self.hint {
                    write!(s, "<label for=\"{}\">{}</label>", id, label)?;
                }
                Ok(())
            }
            InputValue::Truth(truth) => {
                write!(s, "<input")?;
                write!(s, " form=\"{}\"", ctx.current_form())?;
                write!(s, " id=\"{}\"", id)?;
                write!(s, " name=\"{}\"", id)?;
                write!(s, " type=\"checkbox\"")?;
                if *truth {
                    write!(s, " checked")?;
                }
                if readonly {
                    write!(s, " disabled")?;
                }
                write!(s, "/>")?;
                if let Some(label) = &self.hint {
                    write!(s, "<label for=\"{}\">{}</label>", id, label)?;
                }
                Ok(())
            }
            InputValue::Choice(choices, value) => {
                write!(
                    s,
                    "{}",
                    choices
                        .iter()
                        .map(|(title, form)| Ok(format!(
                            "<input type=\"radio\" value=\"{}\" {}/><div class=\"choice\">{}</div>",
                            title,
                            if title == value { "checked" } else { "" },
                            form.as_html(ctx)?,
                        )))
                        .collect::<Result<Vec<_>, _>>()?
                        .join("<br/>")
                )
            }
        }
    }
}
