table! {
    file_assocs (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        dir_id -> Int8,
        child_id -> Int8,
        child_name -> Varchar,
    }
}

table! {
    files (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        kind -> Int2,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        key -> Varchar,
        tree -> Jsonb,
    }
}

allow_tables_to_appear_in_same_query!(file_assocs, files, users,);
