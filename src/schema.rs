table! {
    article (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Varchar,
        thumb -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        category -> Integer,
        excerpt -> Text,
        content -> Text,
        status -> Bool,
        push_front -> Bool,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

table! {
    article_taxonomy_term_map (article_id, taxonomy_term_id) {
        article_id -> Integer,
        taxonomy_term_id -> Integer,
        #[sql_name = "type"]
        type_ -> Varchar,
    }
}

table! {
    taxonomy (id) {
        id -> Integer,
        vid -> Varchar,
        name -> Varchar,
        weight -> Integer,
        description -> Varchar,
    }
}

table! {
    taxonomy_term (id) {
        id -> Integer,
        vid -> Varchar,
        parent -> Integer,
        name -> Varchar,
        machine_name -> Varchar,
        weight -> Integer,
        num -> Integer,
        description -> Varchar,
    }
}

table! {
    user (id) {
        id -> Integer,
        email -> Varchar,
        real_name -> Varchar,
        password_hash -> Varchar,
        password_reset_token -> Varchar,
        is_admin -> Bool,
        created_at -> Integer,
        updated_at -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    article,
    article_taxonomy_term_map,
    taxonomy,
    taxonomy_term,
    user,
);
