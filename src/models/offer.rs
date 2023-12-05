use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::offers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Offer {
    #[serde(default)]
    pub id: String,
    pub customer_id: Option<String>,
    pub name: Option<String>,
    pub date: Option<chrono::NaiveDate>,
    pub customer_text: Option<String>,
    pub subject: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
