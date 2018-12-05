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
    file_owners (id) {
        id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        file_id -> Int8,
        owner_id -> Int8,
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

joinable!(file_owners -> files (file_id));
joinable!(file_owners -> users (owner_id));

allow_tables_to_appear_in_same_query!(
    file_assocs,
    file_owners,
    files,
    users,
);
