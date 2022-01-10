#![cfg(feature = "rocket")]

use std::io::Cursor;

use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::{Request, Response};

use crate::component::Component;
use crate::task::Task;

impl<'r, 'o: 'r, C: Component> Responder<'r, 'o> for Task<C> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let content = format!("<!DOCTYPE html><html><head><title>Hello</title><style>{}</style></head><body>{}</body></html>", include_str!("../static/style.css"), self.content);
        Response::build()
            .header(ContentType::HTML)
            .sized_body(content.len(), Cursor::new(content))
            .ok()
    }
}
