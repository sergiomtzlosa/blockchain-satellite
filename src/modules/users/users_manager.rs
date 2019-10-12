extern crate mysql;

use mysql as my;
use chrono::{NaiveDate, NaiveDateTime};

struct bbdd_time {
    date: NaiveDateTime,
}

pub fn enabled_user(token: &String) -> bool {

    return true;
}

pub fn expired_token(token: &String) -> bool {

    return true;
}
