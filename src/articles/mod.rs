pub mod handler;
pub mod representation;
pub mod persistence;
pub mod mapper;

pub fn routes() -> Vec<rocket::Route> {
  rocket::routes![
    handler::all,
    handler::update,
    handler::delete,
    handler::article,
    handler::up,
    handler::down,
    handler::top,
    handler::bottom,
  ]
}
