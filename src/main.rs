#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "<h1>Hello world!</h1>"
}

#[get("/yay")]
fn yay() -> &'static str {
    "<h1>yay!</h1>"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, yay])
}
