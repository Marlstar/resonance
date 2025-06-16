// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        name -> Text,
        artist -> Nullable<Integer>,
        length -> Integer,
    }
}

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    songs (id) {
        id -> Integer,
        ytid -> Nullable<Text>,
        name -> Text,
        artist -> Nullable<Integer>,
        album -> Nullable<Integer>,
        duration -> Integer,
    }
}

diesel::joinable!(albums -> artists (artist));
diesel::joinable!(songs -> albums (album));
diesel::joinable!(songs -> artists (artist));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    artists,
    songs,
);
