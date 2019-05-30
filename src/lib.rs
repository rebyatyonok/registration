#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::collections::HashMap;

pub mod db;

// public functions 
pub fn create_reg(date: &str, user: &str) -> Result<String, String> {
    let uppercase_user = user.to_uppercase();
    let dates_count = get_dates_count();
    
    match dates_count.get(date) {
        None => Err("There is no such date for registration available".to_string()),
        Some(x) if x >= &6 => Err("Date is full!".to_string()),
        Some(_) => insert_reg(date, uppercase_user.as_str()),
    }
}

pub fn get_all_regs() -> Vec<db::models::Reg> {
    use db::schema::regs::dsl::*;

    let connection = establish_connection();
    let result = regs.load::<db::models::Reg>(&connection)
        .expect("Error loading regs");

    println!("Loaded {} regs", result.len());

    result
}

pub fn get_dates_count() -> HashMap<String, i32> {
    let all_regs = get_all_regs();

    let mut regs_count = HashMap::new();
    
    for reg in all_regs {
        *regs_count.entry(reg.date.clone()).or_insert(0) += 1; 
    }

    regs_count
}

pub fn get_all_dates() -> Vec<db::models::Date> {
    use db::schema::dates::dsl::*;

    let connection = establish_connection();
    let result = dates.load::<db::models::Date>(&connection)
        .expect("Error loading dates");

    println!("Loaded {} dates", result.len());

    result
}

pub fn get_all_users() -> Vec<db::models::User> {
    use db::schema::users::dsl::*;

    let connection = establish_connection();
    let result = users.load::<db::models::User>(&connection)
        .expect("Error loading users");

    println!("Loaded {} users", result.len());

    result
}

// private functions
fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

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

    match result {
        Ok(_) => Ok("Success!".to_string()), 
        Err(e) => Err(e.to_string()),
    }
}
