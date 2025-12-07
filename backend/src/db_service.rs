use crate::schema::customers::{email, member_id, password};
use crate::schema::{self, customers};

use std::env;

use diesel::query_dsl::methods::FilterDsl;
use diesel::result::Error;
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::{PgConnection, r2d2::ConnectionManager};
use dotenv::dotenv;

use crate::model::{Customer, Login, NewCustomer};

type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Dbservice {
    pool: DBPool,
}

impl Dbservice {
    // this sets us up to connect to a db using r2d2
    pub fn new() -> Dbservice {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DB Url must be set...");

        let manager = ConnectionManager::<PgConnection>::new(db_url);

        let result = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to initialise the DB");

        Dbservice { pool: result }
    }

    pub fn create_customer(&self, new_customers: NewCustomer) -> Result<(), Error> {
        diesel::insert_into(schema::customers::table)
            .values(&new_customers)
            .execute(&mut self.pool.get().ok().unwrap())
            .unwrap();

        Ok(())
    }

    pub fn login(&self, login: Login) -> Option<Customer> {
        customers::dsl::customers
            .filter(email.eq(login.email))
            .filter(password.eq(login.password))
            .first::<Customer>(&mut self.pool.get().unwrap())
            .ok()
    }

    pub fn get_by_memberid(&self, id: i32) -> Option<Customer> {
        customers::dsl::customers
            .filter(member_id.eq(id))
            .first::<Customer>(&mut self.pool.get().unwrap())
            .ok()
    }
}
