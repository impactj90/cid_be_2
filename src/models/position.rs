use diesel::{deserialize::Queryable, Selectable, prelude::Insertable, query_builder::AsChangeset};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::positions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Position {
    #[serde(default)]
    pub id: String,
    pub name: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
