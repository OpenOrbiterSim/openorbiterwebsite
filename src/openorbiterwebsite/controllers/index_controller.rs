use rocket::Build;
use rocket_dyn_templates::Template;

pub fn mount(mut rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket = rocket.mount("/", routes![index]);
    rocket
}

#[get("/")]
pub fn index() -> Template {

    Template::render("index",context! {
        site: "banana"
    })
}
