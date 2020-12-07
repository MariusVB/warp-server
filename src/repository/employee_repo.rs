pub mod models;
#[path = "../schema.rs"] pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::models::*;
use schema::employees;
use self::schema::employees::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_all_employees() -> Vec<Employee> {

    let conn = establish_connection();

    return employees
        .load::<Employee>(&conn)
        .expect("Error loading employees");
}

pub fn insert_employee(employee: NewEmployee) -> Employee {

    let conn = establish_connection();

    return diesel::insert_into(employees::table)
        .values(&employee)
        .get_result(&conn)
        .expect("Error saving new employee");
}

pub fn update_employee(employee: Employee) -> Employee {
    let conn = establish_connection();

    return diesel::update(employees.find(employee.id))
    .set(&employee)
    .get_result(&conn)
    .expect("Employee not found")
}

pub fn get_employee(e_id: i32) -> Employee {
    let conn = establish_connection();

    return employees
        .find(e_id)
        .first(&conn)
        .expect("Employee not found")
}

pub fn delete_employee(e_id: i32) -> Employee {
    let conn = establish_connection();

    return diesel::delete(employees.find(e_id))
        .get_result(&conn)
        .expect("Employee not deleted")
}