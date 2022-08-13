#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "hello, world!"
}

#[rocket::main]
async fn main() {
    let _ =  rocket::build()
        .mount("/", routes![index])
        .launch()
        .await
        .expect("panic message");
}

