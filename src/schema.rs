// @generated automatically by Diesel CLI.

diesel::table! {
    cart_items (id) {
        id -> Int4,
        cart_id -> Int4,
        item_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    carts (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Float8,
        stock -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(cart_items -> carts (cart_id));
diesel::joinable!(cart_items -> items (item_id));
diesel::joinable!(carts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(cart_items, carts, items, users,);
