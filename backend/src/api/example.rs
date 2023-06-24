use actix_web::{get, HttpResponse, Responder, Scope};

pub fn example_routes() -> Scope {
    Scope::new("/example").service(hello)
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from Actix!")
}
