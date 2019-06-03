#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::collections::HashMap;

pub mod db;

// public functions

// create a registration on a desired date.
// if there is no such date available or this date has
// 6 or more registrations yet, it will return error
pub fn create_reg(date: &str, user: &str) -> Result<String, String> {
    let uppercase_user = user.to_uppercase();
    let dates_count = get_dates_count();

    // here we should return String instead of str
    // because we want to return diesel error instead of our custom.
    // it implements a Display trait with simple cast to a String type
    match dates_count.get(date) {
        None => Err("There is no such date for registration available".to_string()),
        Some(x) if x >= &6 => Err("Date is full!".to_string()),
        Some(_) => insert_reg(date, uppercase_user.as_str()),
    }
}

// returns array of all registrations inserted.
// it is nedeed only for debug purposes
pub fn get_all_regs() -> Vec<db::models::Reg> {
    use db::schema::regs::dsl::*;

    let connection = establish_connection();
    let result = regs
        .load::<db::models::Reg>(&connection)
        .expect("Error loading regs");

    println!("Loaded {} regs", result.len());

    result
}

// returns all dates available for filtration on client side
pub fn get_all_dates() -> Vec<String> {
    use db::schema::dates::dsl::*;

    let connection = establish_connection();
    let dates_count = get_dates_count();

    // get all dates to filter
    let pre_result = dates
        .load::<db::models::Date>(&connection)
        .expect("Error loading dates");
    
    // filter valid dates
    pre_result.into_iter()
        .filter(|s| dates_count.get(&s.date).unwrap() < &6)
        .map(|s| s.date)
        .collect::<Vec<String>>()
}

// returns all users for a validation on a client side
pub fn get_all_users() -> Vec<db::models::User> {
    use db::schema::users::dsl::*;

    let connection = establish_connection();
    let result = users
        .load::<db::models::User>(&connection)
        .expect("Error loading users");

    println!("Loaded {} users", result.len());

    result
}

// private functions

// creating connection to db
fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// insert registration to db
fn insert_reg<'a>(date: &'a str, user: &'a str) -> Result<String, String> {
    use db::schema::regs;
    let conn = establish_connection();

    let new_reg = db::models::NewReg {
        date: date,
        user: user,
    };

    let result = diesel::insert_into(regs::table)
        .values(&new_reg)
        .execute(&conn);

    // return String types in Result, because we need 
    // the same types in parent function 
    match result {
        Ok(_) => Ok("Success!".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

// return hashmap with date as a key and count of occurences as value
// it is used to filter full dates
fn get_dates_count() -> HashMap<String, i32> {
    let all_regs = get_all_regs();

    let mut regs_count = HashMap::new();

    for reg in all_regs {
        *regs_count.entry(reg.date.clone()).or_insert(0) += 1;
    }

    regs_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_users() {
        let result = get_all_users();

        assert_eq!(6, result.len());
    }

    // test wrong date insertion
    // test all dates method (it should not be changed)
    // test insertion of one right date
    // test all dates method (it should not be changed)
    // test insertion a lot of regs on one date (to make date invalid)
    // test all dates method (it should be changed)
    // test insertion on an invalid date
    // test insertion on another date

    #[test] 
    fn insert_six_valid_registration() {
        for _ in 0..6 {
            let result = create_reg("Tue Oct 01 2019", "Ivan").unwrap();

            assert_eq!(result, "Success!".to_string());
        }
    }
}

