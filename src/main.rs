#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::request::FromFormValue;

#[derive(FromForm)]
struct MyForm {
    field: String, 
}

#[post("/format", data = "<form>")]
fn submit(form: Form<MyForm>) -> String {
    let field = form.field.clone(); 
    form.into_inner();

    return field;
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![submit])
}