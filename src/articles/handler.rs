use rocket::http::Status;
use rocket_contrib::json::Json;
use diesel::prelude::*;
use diesel::pg::upsert::excluded;
use diesel::result::Error;
use crate::Conn;
use crate::schema::articles as repository;
use super::representation::Article;

fn error_status(error: Error) -> Status {
  match error {
      Error::NotFound => Status::NotFound,
      _ => Status::InternalServerError
  }
}

#[get("/")]
pub fn all(conn: Conn) -> Json<Vec<Article>> {
  let dbarticles = repository::table.load::<super::persistence::Article>(&*conn).unwrap(); // This needs ordering by position.
  Json(dbarticles.into_iter().map(|article| article.map()).collect())
}

#[put("/", data="<article>")]
pub fn update(conn: Conn, article: Json<Article>) {
  let dbarticle = article.into_inner().map();
  diesel::insert_into(repository::table).values(dbarticle).on_conflict(repository::id).do_update().set(repository::id.eq(excluded(repository::id))).execute(&*conn).unwrap();
}

#[delete("/?<id>")]
pub fn delete(conn: Conn, id: i32) {
  diesel::delete(repository::table.find(id)).execute(&*conn);
}

#[get("/<id>")]
pub fn article(conn: Conn, id: i32) -> Result<Json<Article>,Status> {
  repository::table.find(id).first(&*conn) 
    .map(|article: super::persistence::Article| Json(article.map()))
    .map_err(|error| error_status(error))  
}

#[put("/up/<id>")]
pub fn up(conn: Conn, id: i32) {
  // Find the article
  // Find the article with the next higher postiion
  // Swap the two
}

#[put("/down/<id>")]
pub fn down(conn: Conn, id: i32) {
  // Find the article
  // Find the article with the next lower postiion
  // Swap the two
}

#[put("/top/<id>")]
pub fn top(conn: Conn, id: i32) {
  // Find the article
  // Find the article with the highest postiion
  // Update the article to have a higher position
}

#[put("/bottom/<id>")]
pub fn bottom(conn: Conn, id: i32) {
  // Find the article
  // Find the article with the lowest postiion
  // Update the article to have a lower position
}
