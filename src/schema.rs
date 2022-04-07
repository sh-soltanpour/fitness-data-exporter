table! {
    daily_activity (id) {
        id -> Int4,
        user_id -> Varchar,
        activity_date -> Date,
        total_steps -> Int4,
        total_distance -> Float4,
        calories -> Int4,
    }
}
