table! {
    config_holidays (id) {
        id -> Int8,
        holiday_date -> Varchar,
        holiday_desc -> Varchar,
        createdat -> Nullable<Timestamp>,
        updatedat -> Nullable<Timestamp>,
    }
}

table! {
    user_login (id) {
        id -> Int8,
        firstname -> Varchar,
        lastname -> Varchar,
        username -> Varchar,
        email -> Varchar,
        mobile -> Varchar,
        facebookconnect -> Nullable<Varchar>,
        googleconnect -> Nullable<Varchar>,
        password -> Varchar,
        ip_address -> Varchar,
        isactive -> Nullable<Bool>,
        sort_order -> Nullable<Int8>,
        created_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Varchar>,
    }
}

allow_tables_to_appear_in_same_query!(config_holidays, user_login,);
