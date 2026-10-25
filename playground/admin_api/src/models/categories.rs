pub use super::_entities::categories::{self, ActiveModel, Entity, Model};
use loco_rs::prelude::*;
pub type Categories = Entity;

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
impl Model {}

// implement your write-oriented logic here
impl ActiveModel {}

// implement your custom finders, selectors oriented logic here
impl Entity {
    /// 后台分类列表的查询规则：`q` 非空时只要名称包含它的记录，按 id 排序。
    pub fn search(q: Option<&str>) -> sea_orm::Select<Self> {
        let select = Self::find().order_by_asc(categories::Column::Id);
        match q.map(str::trim).filter(|q| !q.is_empty()) {
            Some(q) => select.filter(categories::Column::Name.contains(q)),
            None => select,
        }
    }
}
