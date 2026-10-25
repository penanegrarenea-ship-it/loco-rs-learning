pub use super::_entities::products::{self, ActiveModel, Entity, Model};
use loco_rs::prelude::*;
pub type Products = Entity;

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(self, _db: &C, insert: bool) -> std::result::Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if !insert && self.updated_at.is_unchanged() {
            let mut this = self;
            this.updated_at = sea_orm::ActiveValue::Set(chrono::Utc::now().into());
            Ok(this)
        } else {
            Ok(self)
        }
    }
}

// implement your read-oriented logic here
impl Model {
    /// 后台“在售商品”列表：领域规则（什么算在售）写在 model 上，controller 只调用。
    pub async fn list_on_sale(db: &DatabaseConnection) -> ModelResult<Vec<Self>> {
        let items = Entity::find()
            .filter(products::Column::Status.eq("on_sale"))
            .order_by_asc(products::Column::Id)
            .all(db)
            .await?;
        Ok(items)
    }
}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {}
