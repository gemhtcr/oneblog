pub use sea_orm_migration::prelude::*;

mod m20230725_000001_create_category_table;
mod m20230725_000002_create_post_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230725_000001_create_category_table::Migration),
            Box::new(m20230725_000002_create_post_table::Migration)
        ]
    }
}
