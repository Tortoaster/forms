use std::io::Cursor;

use rocket::fairing::{Fairing, Info, Kind};
#[cfg(debug_assertions)]
use rocket::fs::{relative, NamedFile};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{get, routes, Build, Request, Response, Rocket};

use crate::component::Component;
use crate::task::Task;

impl<'r, 'o: 'r, C: Component> Responder<'r, 'o> for Task<C> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'o> {
        let content = format!(
            "<!DOCTYPE html><html><head><title>iTasks</title><link rel=\"stylesheet\" href=\"/itasks/style.css\"/></head><body><div class=\"container\">{}</div></body></html>",
            self.form.html().map_err(|_| Status::InternalServerError)?
        );
        Response::build()
            .header(ContentType::HTML)
            .sized_body(content.len(), Cursor::new(content))
            .ok()
    }
}

pub struct ITasks;

#[cfg(not(debug_assertions))]
#[get("/style.css")]
async fn style() -> &'static str {
    include_str!("../frontend/style.css")
}

#[cfg(debug_assertions)]
#[get("/style.css")]
async fn style() -> Result<NamedFile, Status> {
    NamedFile::open(relative!("./src/frontend/style.css"))
        .await
        .map_err(|_| Status::NotFound)
}

#[rocket::async_trait]
impl Fairing for ITasks {
    fn info(&self) -> Info {
        Info {
            name: "iTasks",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        Ok(rocket.mount("/itasks", routes![style]))
    }
}
