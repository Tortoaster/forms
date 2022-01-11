use std::io::Cursor;

use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::{Request, Response};

use crate::component::Component;
use crate::task::Task;

impl<'r, 'o: 'r, C: Component> Responder<'r, 'o> for Task<C> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let content = format!("<!DOCTYPE html><html><head><title>Hello</title><style>{}</style></head><body>{}</body></html>", include_str!("../frontend/style.css"), self.form.as_html());
        Response::build()
            .header(ContentType::HTML)
            .sized_body(content.len(), Cursor::new(content))
            .ok()
    }
}
