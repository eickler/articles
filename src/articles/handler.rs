use rocket_contrib::json::Json;
use diesel::prelude::*;
use crate::Conn;
use crate::schema::articles as dbarticles;
use super::representation::Article;

#[get("/")]
pub fn all(conn: Conn) -> Json<Article> {
  let articles = dbarticles::table.load::<super::persistence::Article>(&*conn);
  // Map now to other datatype, maybe in mapper.

  // let article = Article {
  //   id: 0,
  //   categories: vec![0, 2],
  //   author: String::from("Wahnfried Willkür"),
  //   title: String::from("Total digital"),
  //   created: Local::now(),
  //   tags: vec![String::from("Spannend"), String::from("Grün")],
  //   teaser: String::from("something.jpg"),
  //   content: String::from("You think water moves fast? You should see ice. It moves like it has a mind. Like it knows it killed the world once and got a taste for murder."),
  //   draft: false,
  //   video_file_name: String::from(""),
  //   abstract_: String::from("")
  // };
  // Json(article)
}

#[put("/", data="<article>")]
pub fn update(conn: Conn, article: Json<Article>) {
}

#[delete("/?<id>")]
pub fn delete(conn: Conn, id: u32) {

}

#[get("/<id>")]
pub fn article(conn: Conn, id: u32) {

}

#[put("/up/<id>")]
pub fn up(conn: Conn, id: u32) {

}

#[put("/down/<id>")]
pub fn down(conn: Conn, id: u32) {

}

#[put("/top/<id>")]
pub fn top(conn: Conn, id: u32) {

}

#[put("/bottom/<id>")]
pub fn bottom(conn: Conn, id: u32) {

}
