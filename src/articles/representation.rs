use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Debug)]
pub struct Article {
  pub id: i32,
  pub categories: Vec<i32>,
  pub author: String,
  pub title: String,
  pub tags: Vec<String>,
  #[serde(rename = "abstract")]
  pub abstract_: String,
  pub teaser: String,
  pub content: String,
  pub draft: bool,
  pub created: DateTime<Local>,
  #[serde(rename = "videoFileName")]
  pub video_file_name: String,
}
