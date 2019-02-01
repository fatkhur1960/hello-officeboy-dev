table! {
    accounts (id) {
        id -> Int8,
        full_name -> Varchar,
        balance -> Float8,
        email -> Varchar,
        phone_num -> Varchar,
        active -> Bool,
        register_time -> Timestamp,
    }
}

table! {
    addresses (id) {
        id -> Int8,
        account_id -> Int8,
        kind -> Int4,
        address -> Text,
        regency -> Varchar,
        province -> Varchar,
        country -> Varchar,
        phone_num -> Varchar,
        active -> Bool,
        notes -> Text,
    }
}

table! {
    register_accounts (id) {
        id -> Int8,
        full_name -> Varchar,
        email -> Varchar,
        phone_num -> Varchar,
        register_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(accounts, addresses, register_accounts,);
