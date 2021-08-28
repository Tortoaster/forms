pub trait Component {
    fn view(&self) -> String;

    fn enter() -> String;

    fn update(&mut self) -> String;
}

impl Component for bool {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"checkbox\"/>")
    }

    fn update(&mut self) -> String {
        format!("<input type=\"checkbox\"{}/>", if *self { " checked" } else { "" })
    }
}

impl Component for isize {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for usize {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for i8 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for u8 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for i16 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for u16 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for i32 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for u32 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for i64 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for u64 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for i128 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for u128 {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\"/>", Self::MIN, Self::MAX)
    }

    fn update(&mut self) -> String {
        format!("<input type=\"number\" min=\"{}\" max=\"{}\" value=\"{}\"/>", Self::MIN, Self::MAX, self)
    }
}

impl Component for char {
    fn view(&self) -> String {
        self.to_string()
    }

    fn enter() -> String {
        format!("<input type=\"text\" maxlength=\"1\"/>")
    }

    fn update(&mut self) -> String {
        format!("<input type=\"text\" maxlength=\"1\" value=\"{}\"/>", self)
    }
}

impl Component for &str {
    fn view(&self) -> String {
        format!("<span>{}</span>", self)
    }

    fn enter() -> String {
        format!("<input type=\"text\"/>")
    }

    fn update(&mut self) -> String {
        format!("<input type=\"text\" value=\"{}\"/>", self)
    }
}

impl Component for String {
    fn view(&self) -> String {
        format!("<span>{}</span>", self)
    }

    fn enter() -> String {
        format!("<input type=\"text\"/>")
    }

    fn update(&mut self) -> String {
        format!("<input type=\"text\" value=\"{}\"/>", self)
    }
}

impl Component for () {
    fn view(&self) -> String {
        String::new()
    }

    fn enter() -> String {
        String::new()
    }

    fn update(&mut self) -> String {
        String::new()
    }
}

impl<C1> Component for (C1, )
    where C1: Component {
    fn view(&self) -> String {
        format!("<div class=\"component\">{}</div>", self.0.view())
    }

    fn enter() -> String {
        format!("<div class=\"component\">{}</div>", C1::enter())
    }

    fn update(&mut self) -> String {
        format!("<div class=\"component\">{}</div>", self.0.update())
    }
}

impl<C1, C2> Component for (C1, C2)
    where C1: Component,
          C2: Component {
    fn view(&self) -> String {
        format!("<div class=\"component\">{}<hr/>{}</div>", self.0.view(), self.1.view())
    }

    fn enter() -> String {
        format!("<div class=\"component\">{}<hr/>{}</div>", C1::enter(), C2::enter())
    }

    fn update(&mut self) -> String {
        format!("<div class=\"component\">{}<hr/>{}</div>", self.0.update(), self.1.update())
    }
}

impl<C1, C2, C3> Component for (C1, C2, C3)
    where C1: Component,
          C2: Component,
          C3: Component {
    fn view(&self) -> String {
        format!("<div class=\"component\">{}<hr/>{}<hr/>{}</div>", self.0.view(), self.1.view(), self.2.view())
    }

    fn enter() -> String {
        format!("<div class=\"component\">{}<hr/>{}<hr/>{}</div>", C1::enter(), C2::enter(), C3::enter())
    }

    fn update(&mut self) -> String {
        format!("<div class=\"component\">{}<hr/>{}<hr/>{}</div>", self.0.update(), self.1.update(), self.2.update())
    }
}

impl<C1, C2, C3, C4> Component for (C1, C2, C3, C4)
    where C1: Component,
          C2: Component,
          C3: Component,
          C4: Component {
    fn view(&self) -> String {
        format!("<div class=\"component\">{}<hr/>{}<hr/>{}<hr/>{}</div>", self.0.view(), self.1.view(), self.2.view(), self.3.view())
    }

    fn enter() -> String {
        format!("<div class=\"component\">{}<hr/>{}<hr/>{}<hr/>{}</div>", C1::enter(), C2::enter(), C3::enter(), C4::enter())
    }

    fn update(&mut self) -> String {
        format!("<div class=\"component\">{}<hr/>{}<hr/>{}<hr/>{}</div>", self.0.update(), self.1.update(), self.2.update(), self.3.update())
    }
}
