pub mod representation;
pub mod handler;
pub mod persistence;

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
