use std::fmt::Write;

use crate::frontend::form::{Form, Input, InputValue};

impl Form {
    pub fn as_html(&self) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        self.write_html(&mut s)?;
        Ok(s)
    }

    fn write_html(&self, s: &mut String) -> std::fmt::Result {
        write!(s, "<div class=\"component\">")?;

        if let Some(title) = &self.title {
            write!(s, "<div class=\"title\">{}</div>", title)?;
        }

        write!(s, "<div class=\"content\">")?;
        write!(
            s,
            "{}",
            self.inputs
                .iter()
                .map(|input| input.as_html(self.readonly))
                .collect::<Result<Vec<_>, _>>()?
                .join("<br/>")
        )?;
        write!(s, "</div>")?;

        write!(s, "</div>")
    }
}

impl Input {
    fn as_html(&self, readonly: bool) -> Result<String, std::fmt::Error> {
        let mut s = String::new();
        self.write_html(&mut s, readonly)?;
        Ok(s)
    }

    fn write_html(&self, s: &mut String, readonly: bool) -> std::fmt::Result {
        match &self.value {
            InputValue::Form(form) => {
                if let Some(hint) = &self.hint {
                    write!(s, "{}: ", hint)?;
                }
                write!(s, "{}", form.as_html()?)
            }
            InputValue::Text(text) => {
                write!(s, "<input")?;
                if let Some(hint) = &self.hint {
                    write!(s, " placeholder=\"{}\"", hint)?;
                }
                write!(s, " type=\"text\"")?;
                write!(s, " value=\"{}\"", text)?;
                if readonly {
                    write!(s, " readonly")?;
                }
                write!(s, "/>")
            }
            InputValue::Character(character) => {
                write!(s, "<input")?;
                if let Some(hint) = &self.hint {
                    write!(s, " placeholder=\"{}\"", hint)?;
                }
                write!(s, " type=\"text\"")?;
                write!(s, " value=\"{}\"", character)?;
                write!(s, " max_length=\"1\"")?;
                if readonly {
                    write!(s, " readonly")?;
                }
                write!(s, "/>")
            }
            InputValue::Number(number) => {
                write!(s, "<input")?;
                if let Some(hint) = &self.hint {
                    write!(s, " placeholder=\"{}\"", hint)?;
                }
                write!(s, " type=\"number\"")?;
                write!(s, " value=\"{}\"", number)?;
                if readonly {
                    write!(s, " readonly")?;
                }
                write!(s, "/>")
            }
            InputValue::Truth(truth) => match &self.hint {
                None => {
                    write!(s, "<input")?;
                    write!(s, " type=\"checkbox\"")?;
                    if *truth {
                        write!(s, " checked")?;
                    }
                    if readonly {
                        write!(s, " readonly")?;
                    }
                    write!(s, "/>")
                }
                Some(hint) => {
                    write!(s, "<label>{}: ", hint)?;
                    write!(s, "<input")?;
                    write!(s, " type=\"checkbox\"")?;
                    if *truth {
                        write!(s, " checked")?;
                    }
                    if readonly {
                        write!(s, " readonly")?;
                    }
                    write!(s, "/>")?;
                    write!(s, "</label>")
                }
            },
        }
    }
}
