use ts_rs::TS;
use validator::Validate;

#[derive(serde::Serialize, serde::Deserialize, TS)]
pub struct CategoryDto {
    #[ts(type = "number")]
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

impl From<crate::models::_entities::categories::Model> for CategoryDto {
    fn from(m: crate::models::_entities::categories::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            description: m.description,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, TS, Validate)]
pub struct CreateCategory {
    /// 没传 name 时按 "" 处理，走字段级校验，而不是笼统的 422
    #[serde(default)]
    #[validate(length(min = 1, max = 50, message = "name must be 1-50 characters"))]
    pub name: String,
    #[validate(length(max = 200, message = "description must be at most 200 characters"))]
    pub description: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, TS, Validate)]
pub struct UpdateCategory {
    /// 没传 name 时按 "" 处理，走字段级校验，而不是笼统的 422
    #[serde(default)]
    #[validate(length(min = 1, max = 50, message = "name must be 1-50 characters"))]
    pub name: String,
    #[validate(length(max = 200, message = "description must be at most 200 characters"))]
    pub description: Option<String>,
}
