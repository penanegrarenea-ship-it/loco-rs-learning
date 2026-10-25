use sea_orm::prelude::DateTimeWithTimeZone;
use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, TS)]
pub struct CategoryDto {
    #[ts(type = "number")]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    #[ts(type = "string")]
    pub created_at: DateTimeWithTimeZone,
    #[ts(type = "string")]
    pub updated_at: DateTimeWithTimeZone,
}

impl From<crate::models::_entities::categories::Model> for CategoryDto {
    fn from(m: crate::models::_entities::categories::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            description: m.description,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, TS)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, TS)]
pub struct UpdateCategory {
    pub name: String,
    pub description: Option<String>,
}
