use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::offer_services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OfferService {
    #[serde(default)]
    pub id: String,
    pub offer_id: Option<String>,
    pub service_id: Option<String>,
    pub position_id: Option<String>,

    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
