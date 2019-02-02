table! {
    access_tokens (token) {
        token -> Text,
        account_id -> Int8,
        created -> Timestamp,
        valid_thru -> Timestamp,
    }
}

table! {
    account_passhash (account_id) {
        account_id -> Int8,
        passhash -> Varchar,
        deprecated -> Bool,
        created -> Timestamp,
    }
}

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
    invoice_items (id) {
        id -> Int8,
        invoice_id -> Int8,
        name -> Varchar,
        price -> Float8,
    }
}

table! {
    invoices (id) {
        id -> Int8,
        id_ref -> Text,
        issuer_account -> Int8,
        to_account -> Int8,
        discount -> Float8,
        amount -> Float8,
        notes -> Text,
        created -> Timestamp,
        paid -> Bool,
        paid_by -> Int8,
        paid_at -> Nullable<Timestamp>,
    }
}

table! {
    payment_history (id) {
        id -> Int8,
        invoice_id -> Int8,
        payer -> Int8,
        via -> Varchar,
        ts -> Timestamp,
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

joinable!(access_tokens -> accounts (account_id));
joinable!(account_passhash -> accounts (account_id));
joinable!(invoice_items -> invoices (invoice_id));
joinable!(invoices -> accounts (paid_by));
joinable!(payment_history -> accounts (payer));
joinable!(payment_history -> invoices (invoice_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    account_passhash,
    accounts,
    addresses,
    invoice_items,
    invoices,
    payment_history,
    register_accounts,
);
