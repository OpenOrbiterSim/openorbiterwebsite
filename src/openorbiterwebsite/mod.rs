use rocket_dyn_templates::Template;
use rocket::fs::{FileServer, relative};

pub mod controllers;

pub async fn run() -> Result<(), rocket::Error> {
    let mut rocket = rocket::build()
        .attach(Template::fairing());

    rocket = controllers::index_controller::mount(rocket);

    rocket = rocket.mount("/", FileServer::from(relative!("static")));

    rocket.launch().await?;

    Ok(())
}
