use crate::schema::customers;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

// this the entire table struct (all table fields)
#[derive(Queryable, Debug, Serialize)]
pub struct Customer {
    pub member_id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
}

impl Customer {
    pub fn to_response(&self) -> ProfileDetails {
        ProfileDetails {
            name: self.name.clone(),
            email: self.email.clone(),
        }
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = customers)] // this has to match the table name
pub struct NewCustomer {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct ProfileDetails {
    pub name: String,
    pub email: String,
}
