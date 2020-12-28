use chrono::Local;

impl super::representation::Article {
  pub fn map(self) -> super::persistence::Article {
    // TODO  This needs to do validation
    // TODO This needs handling of position
    super::persistence::Article {
      id: self.id,
      categories: self.categories,
      author: self.author,
      title: self.title,
      tags: self.tags,
      abstract_: self.abstract_,
      teaser: self.teaser,
      articles_content: self.content,
      draft: self.draft,
      last_update: Local::now().naive_local(),
      position: 0,
      video_file_name: self.video_file_name,
    }
  }
}

impl super::persistence::Article {
  pub fn map(self) -> super::representation::Article {
    super::representation::Article {
      id: self.id,
      categories: self.categories,
      author: self.author,
      title: self.title,
      tags: self.tags,
      abstract_: self.abstract_,
      teaser: self.teaser,
      content: self.articles_content,
      draft: self.draft,
      created: self.last_update,
      video_file_name: self.video_file_name
    }
  }
}
