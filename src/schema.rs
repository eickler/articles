table! {
    articles (id) {
        id -> Int4,
        categories -> Nullable<Array<Int4>>,
        author -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        tags -> Nullable<Array<Varchar>>, // Diesel recommends setting this to Array<Text> instead.
        #[sql_name = "abstract"]
        abstract_ -> Nullable<Text>,
        teaser -> Nullable<Varchar>,
        articles_content -> Nullable<Text>,
        draft -> Nullable<Bool>,
        last_update -> Timestamp,
        position -> Nullable<Int4>,
        video_file_name -> Nullable<Varchar>,
    }
}
