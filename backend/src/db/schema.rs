// @generated automatically by Diesel CLI.

diesel::table! {
    playlist (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    playlist_entry (id) {
        id -> Integer,
        idx -> Integer,
        song_id -> Integer,
        playlist_id -> Integer,
    }
}

diesel::table! {
    song (id) {
        id -> Integer,
        ytid -> Text,
        name -> Text,
        author -> Text,
        duration -> Integer,
        album -> Text,
    }
}

diesel::joinable!(playlist_entry -> playlist (playlist_id));
diesel::joinable!(playlist_entry -> song (song_id));

diesel::allow_tables_to_appear_in_same_query!(
    playlist,
    playlist_entry,
    song,
);
