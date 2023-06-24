use actix_web::Scope;

pub mod example;

pub fn routes() -> Scope {
    Scope::new("/api").service(example::example_routes())
}
