table! {
    address (id) {
        id -> Int4,
        city -> Varchar,
        street -> Varchar,
        number -> Int4,
        zip -> Bpchar,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        parent_id -> Nullable<Int4>,
    }
}

table! {
    categorizations (id) {
        id -> Int4,
        category_id -> Int4,
        product_id -> Int4,
    }
}

table! {
    mystate (id) {
        id -> Int4,
        my_state -> Varchar,
    }
}

table! {
    orders (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        city -> Varchar,
        street -> Varchar,
        number -> Int4,
        zip -> Bpchar,
        name -> Varchar,
        price -> Int4,
        state_id -> Int4,
    }
}

table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        price -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        address_id -> Int4,
    }
}

joinable!(categorizations -> categories (category_id));
joinable!(categorizations -> products (product_id));
joinable!(orders -> mystate (state_id));
joinable!(users -> address (address_id));

allow_tables_to_appear_in_same_query!(
    address,
    categories,
    categorizations,
    mystate,
    orders,
    products,
    users,
);
