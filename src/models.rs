use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Account {
    pub id: i64,
    pub full_name: String,
    pub balance: f64,
    pub email: String,
    pub phone_num: String,
    pub active: bool,
    pub register_time: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Address {
    pub id: i64,
    pub account_id: i64,
    pub kind: i64,
    pub address: String,
    pub regency: String,
    pub province: String,
    pub country: String,
    pub phone_num: String,
    pub active: bool,
    pub notes: String,
}

#[derive(Queryable)]
pub struct RegisterAccount {
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub phone_num: String,
    pub register_time: NaiveDateTime,
}
