table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        author_id -> Int4,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(posts -> users (author_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
