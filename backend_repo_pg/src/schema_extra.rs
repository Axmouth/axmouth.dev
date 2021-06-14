table! {
    use diesel::sql_types::*;
    use diesel_full_text_search::TsVector;
    use crate::exports::*;

    search_items (title) {
        id -> Varchar,
        search_vec -> TsVector,
        title -> Varchar,
        description -> Varchar,
        item_type -> Search_item_type,
        link -> Varchar,
    }
}
