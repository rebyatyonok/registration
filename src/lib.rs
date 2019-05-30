#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use std::collections::HashMap;

pub mod db;

// public functions 
pub fn create_reg(date: &str, user: &str) -> Result<usize, &'static str> {
    let conn = establish_connection();
    let uppercase_user = user.to_uppercase();

    match get_valid_dates().iter().position(|s| s.date == date) {
        Some(_) => Ok(insert_reg(&conn, date, uppercase_user.as_str())), 
        None => Err("Date is not valid"),
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

pub fn get_valid_dates() -> Vec<db::models::Date> {
    let all_dates = get_all_dates();
    let all_regs = get_all_regs();

    if all_regs.len() < 6 {
        return all_dates;
    }
    
    let mut regs_count = HashMap::new();
    
    for reg in all_regs {
        *regs_count.entry(reg.date.clone()).or_insert(0) += 1; 
    }
    
    all_dates.into_iter()
        .filter(|date| regs_count[&date.date] < 6).collect()
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

fn insert_reg<'a>(conn: &SqliteConnection, date: &'a str, user: &'a str) -> usize {
    use db::schema::regs;

    let new_reg = db::models::NewReg {
        date: date,
        user: user,
    };

    diesel::insert_into(regs::table)
        .values(&new_reg)
        .execute(conn)
        .expect("Error saving new reg")
}


