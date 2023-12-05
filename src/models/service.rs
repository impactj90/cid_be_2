use bigdecimal::BigDecimal;
use diesel::{deserialize::Queryable, prelude::Insertable, Selectable, query_builder::AsChangeset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Service {
    #[serde(default)]
    pub id: String,
    pub service_text: String,
    pub quantity_unit: String,
    pub quantity: i32,
    pub price_per_unit: BigDecimal,
    pub total_cost: BigDecimal,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
