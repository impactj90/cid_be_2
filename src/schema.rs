// @generated automatically by Diesel CLI.

diesel::table! {
    customers (id) {
        id -> Int4,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        #[max_length = 10]
        zip -> Nullable<Varchar>,
        #[max_length = 50]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        street -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int8,
        login_timestamp -> Timestamptz,
    }
}

diesel::table! {
    offer_services (id) {
        id -> Int4,
        offer_id -> Int4,
        service_id -> Int4,
        position_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    offers (id) {
        id -> Int4,
        customer_id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        date -> Nullable<Date>,
        customer_text -> Nullable<Text>,
        #[max_length = 255]
        subject -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Bool,
        age -> Int4,
        address -> Varchar,
        #[max_length = 11]
        phone -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    positions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    services (id) {
        id -> Int4,
        #[max_length = 255]
        service_text -> Varchar,
        #[max_length = 50]
        quantity_unit -> Varchar,
        quantity -> Int4,
        price_per_unit -> Numeric,
        total_cost -> Numeric,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

diesel::joinable!(login_history -> users (user_id));
diesel::joinable!(offer_services -> offers (offer_id));
diesel::joinable!(offer_services -> positions (position_id));
diesel::joinable!(offer_services -> services (service_id));
diesel::joinable!(offers -> customers (customer_id));

diesel::allow_tables_to_appear_in_same_query!(
    customers,
    login_history,
    offer_services,
    offers,
    people,
    positions,
    services,
    users,
);
