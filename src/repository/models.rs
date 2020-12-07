use serde::{Deserialize, Serialize};
use super::schema::employees;

#[derive(Identifiable, AsChangeset, Queryable, Deserialize, Serialize, Clone)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub rate: i32,
}

#[derive(Insertable, Deserialize, Serialize, Clone)]
#[table_name="employees"]
pub struct NewEmployee {
    pub name: String,
    pub rate: i32,
}