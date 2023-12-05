use diesel::{
    deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, QueryResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    schema::customers::{self, dsl::*},
};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Customer {
    pub id: i32,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub street: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = customers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CustomerDTO {
    pub id: i32,
    pub phone: Option<String>,
    pub name: Option<String>,
    pub zip: Option<String>,
    pub city: Option<String>,
    pub street: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Customer {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Customer>> {
        customers.order(id.asc()).load::<Customer>(conn)
    }

    pub fn find_by_id(i: i32, conn: &mut Connection) -> QueryResult<Customer> {
        customers.find(i).get_result::<Customer>(conn)
    }

    pub fn insert(new_person: CustomerDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(customers)
            .values(&new_person)
            .execute(conn)
    }

    pub fn update(
        i: i32,
        updated_person: CustomerDTO,
        conn: &mut Connection,
    ) -> QueryResult<usize> {
        diesel::update(customers.find(i))
            .set(&updated_person)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(customers.find(i)).execute(conn)
    }
}
