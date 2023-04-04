#[macro_use] extern crate rocket;

#[get("/add")]
fn add() -> &'static str {
    "Hey! Let's add numbers!"
}

#[get("/add/<a>")]
fn add_a(a: u32) -> String {
    format!("Hey! Let's add! Your first input is here: {} but you haven't put another number", a)
}

#[get("/add/<a>/<b>")]
fn add_a_b(a: u32, b: u32) -> String {
    format!("Hey! Let's add! Your inputs are: {} and {}. The sum is {}!", a, b, a + b)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add, add_a, add_a_b])
}
