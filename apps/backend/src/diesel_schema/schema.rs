// @generated automatically by Diesel CLI.

// Presents the schema of the users table in the database.
// The schema! macro is used to define the structure of the users table.
diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        password -> Varchar,
        #[max_length = 50]
        email -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}
