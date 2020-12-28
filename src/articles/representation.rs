use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
  pub id: i32,
  pub categories: Vec<i32>,
  pub author: String,
  pub title: String,
  pub tags: Vec<String>,
  #[serde(rename = "abstract")]
  pub abstract_: String,
  pub teaser: Option<String>,
  pub content: String,
  pub draft: bool,
  pub created: NaiveDateTime,
  #[serde(rename = "videoFileName")]
  pub video_file_name: Option<String>,
}
