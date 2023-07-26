use crate::m20230725_000001_create_category_table::Category;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let table = Table::create()
            // Table
            .table(Post::Table)
            .if_not_exists()
            // Id
            .col(
                ColumnDef::new(Post::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            // Title
            .col(ColumnDef::new(Post::Title).string().not_null())
            // Description
            .col(ColumnDef::new(Post::Description).string().not_null())
            // CategoryId
            .col(ColumnDef::new(Post::CategoryId).integer().null())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_2e303c3a712662f1fc2a4d0aad6")
                    .from(Post::Table, Post::CategoryId)
                    .to(Category::Table, Category::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .col(ColumnDef::new(Post::Created).timestamp().not_null())
            .col(ColumnDef::new(Post::Updated).timestamp().not_null())
            .to_owned();
        println!("{:?}", table.to_string(MysqlQueryBuilder));
        manager.create_table(table).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    Title,
    Description,
    CategoryId,
    Created,
    Updated,
}
