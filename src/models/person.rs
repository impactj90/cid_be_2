use diesel::{prelude::*, AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    schema::people::{self, dsl::*},
};


#[derive(Queryable, Serialize, Deserialize)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = people)]
pub struct PersonDTO {
    pub name: String,
    pub gender: bool,
    pub age: i32,
    pub address: String,
    pub phone: String,
    pub email: String,
}

impl Person {
    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Person>> {
        people.order(id.asc()).load::<Person>(conn)
    }

    pub fn find_by_id(i: i32, conn: &mut Connection) -> QueryResult<Person> {
        people.find(i).get_result::<Person>(conn)
    }

    pub fn insert(new_person: PersonDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(people)
            .values(&new_person)
            .execute(conn)
    }

    pub fn update(i: i32, updated_person: PersonDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::update(people.find(i))
            .set(&updated_person)
            .execute(conn)
    }

    pub fn delete(i: i32, conn: &mut Connection) -> QueryResult<usize> {
        diesel::delete(people.find(i)).execute(conn)
    }
}
