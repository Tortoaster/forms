#![cfg(feature = "rocket")]

use std::io::Cursor;

use rocket::{Request, Response};
use rocket::http::ContentType;
use rocket::response::Responder;

use crate::{Component, Task};

impl<'r, 'o: 'r, T: Component> Responder<'r, 'o> for Task<T> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let content = format!("<!DOCTYPE html><html><head><title>Hello</title></head><body>{}</body></html>", self.view());
        Response::build()
            .header(ContentType::HTML)
            .sized_body(content.len(), Cursor::new(content))
            .ok()
    }
}