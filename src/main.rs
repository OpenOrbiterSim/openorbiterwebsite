#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_dyn_templates;

pub mod openorbiterwebsite;

#[rocket::main]
async fn main() {
    openorbiterwebsite::run().await;
}
