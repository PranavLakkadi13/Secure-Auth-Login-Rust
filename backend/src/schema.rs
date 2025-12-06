// @generated automatically by Diesel CLI.

diesel::table! {
    customers (member_id) {
        member_id -> Int4,
        name -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}
