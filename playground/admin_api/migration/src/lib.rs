#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20261025_123659_products;
mod m20261025_134358_categories;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20261025_123659_products::Migration),
            Box::new(m20261025_134358_categories::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
