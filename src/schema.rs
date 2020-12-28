table! {
    articles (id) {
        id -> Int4,
        categories -> Array<Int4>,
        author -> Varchar,
        title -> Varchar,
        tags -> Array<Varchar>, // Diesel recommends setting this to Array<Text> instead.
        #[sql_name = "abstract"]
        abstract_ -> Text,
        teaser -> Nullable<Varchar>,
        articles_content -> Text,
        draft -> Bool,
        last_update -> Timestamp,
        position -> Int4,
        video_file_name -> Nullable<Varchar>,
    }
}
