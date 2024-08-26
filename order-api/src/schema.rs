// @generated automatically by Diesel CLI.

diesel::table! {
    cart (id) {
        id -> Int4,
        cart_id -> Uuid,
        order_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    order_status (id) {
        id -> Int2,
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        order_id -> Uuid,
        payment_id -> Nullable<Uuid>,
        order_status_id -> Nullable<Int2>,
        address -> Nullable<Varchar>,
        shipment_id -> Nullable<Uuid>,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
        description -> Nullable<Varchar>,
        total_price -> Nullable<Numeric>,
        cart_id -> Uuid,
    }
}

diesel::table! {
    product_ledger (id) {
        id -> Int4,
        cart_id -> Uuid,
        product_id -> Nullable<Uuid>,
        quantity -> Nullable<Int2>,
        price_per_unit -> Nullable<Numeric>,
        cart_status_id -> Nullable<Int2>,
        discount -> Nullable<Numeric>,
        created_at -> Nullable<Timestamp>,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    product_ledger_status (id) {
        id -> Int2,
        name -> Nullable<Varchar>,
    }
}

diesel::joinable!(orders -> order_status (order_status_id));
diesel::joinable!(product_ledger -> product_ledger_status (cart_status_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart,
    order_status,
    orders,
    product_ledger,
    product_ledger_status,
);
