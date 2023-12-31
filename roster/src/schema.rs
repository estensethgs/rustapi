// @generated automatically by Diesel CLI.

diesel::table! {
    alts (id) {
        id -> Int4,
        name -> Text,
        player_id -> Int4,
    }
}

diesel::table! {
    players (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::joinable!(alts -> players (player_id));

diesel::allow_tables_to_appear_in_same_query!(
    alts,
    players,
);
