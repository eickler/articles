use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Article {
  pub id: i32,
  pub categories: Vec<i32>,
  pub author: String,
  pub title: String,
  pub tags: Vec<String>,
  pub abstract_: String,
  pub teaser: Option<String>,
  pub articles_content: String,
  pub draft: bool,
  pub last_update: NaiveDateTime,
  pub position: i32,
  pub video_file_name: Option<String>,
}
