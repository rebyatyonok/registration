use super::schema::{regs};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Queryable)]
pub struct Date {
    pub id: i32,
    pub date: String,
}

#[derive(Queryable)]
pub struct Reg {
    pub id: i32,
    pub date: String,
    pub user: String,
}

#[derive(Insertable)]
#[table_name="regs"]
pub struct NewReg<'a> {
    pub date: &'a str,
    pub user: &'a str,
}
