pub trait Component {
    fn view(&self) -> String;
}

impl Component for bool {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for isize {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for usize {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for i8 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for u8 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for i16 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for u16 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for i32 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for u32 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for i64 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for u64 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for i128 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for u128 {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for char {
    fn view(&self) -> String {
        self.to_string()
    }
}

impl Component for &str {
    fn view(&self) -> String {
        format!("<span>{}</span>", self)
    }
}

impl Component for String {
    fn view(&self) -> String {
        format!("<span>{}</span>", self)
    }
}

impl Component for () {
    fn view(&self) -> String {
        String::new()
    }
}

impl<C1> Component for (C1, )
    where C1: Component {
    fn view(&self) -> String {
        format!("<div>{}</div>", self.0.view())
    }
}

impl<C1, C2> Component for (C1, C2)
    where C1: Component,
          C2: Component {
    fn view(&self) -> String {
        format!("<div>{}<br/>{}</div>", self.0.view(), self.1.view())
    }
}

impl<C1, C2, C3> Component for (C1, C2, C3)
    where C1: Component,
          C2: Component,
          C3: Component {
    fn view(&self) -> String {
        format!("<div>{}<br/>{}<br/>{}</div>", self.0.view(), self.1.view(), self.2.view())
    }
}

impl<C1, C2, C3, C4> Component for (C1, C2, C3, C4)
    where C1: Component,
          C2: Component,
          C3: Component,
          C4: Component {
    fn view(&self) -> String {
        format!("<div>{}<br/>{}<br/>{}<br/>{}</div>", self.0.view(), self.1.view(), self.2.view(), self.3.view())
    }
}
