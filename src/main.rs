#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[get("/about")]
fn about() -> &'static str {
    "This is the about page of my Rocket Demo App"
}

#[rocket::main]
async fn main() {
    let _ =  rocket::build()
        .mount("/", routes![index, about])
        .launch()
        .await
        .expect("panic message");
}

