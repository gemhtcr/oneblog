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
            .col(ColumnDef::new(Post::CategoryName).string().null())
            .foreign_key(
                ForeignKey::create()
                    .name("fk_2e303c3a712662f1fc2a4d0aad6")
                    .from(Post::Table, Post::CategoryName)
                    .to(Category::Table, Category::Name)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .col(ColumnDef::new(Post::Created).timestamp().not_null())
            .col(ColumnDef::new(Post::Updated).timestamp().not_null())
            .to_owned();
        manager.create_table(table).await?;
        let insert = Query::insert()
            .into_table(Post::Table)
            .columns([
                Post::Title,
                Post::Description,
                Post::CategoryName,
                Post::Created,
                Post::Updated,
            ])
            .values_panic([
                "Title 1".into(),
                "description 1".into(),
                "Category1".into(),
                chrono::offset::Utc::now().into(),
                chrono::offset::Utc::now().into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(Post::Table)
            .columns([
                Post::Title,
                Post::Description,
                Post::CategoryName,
                Post::Created,
                Post::Updated,
            ])
            .values_panic([
                "Title 2".into(),
                "description 2".into(),
                "Category2".into(),
                chrono::offset::Utc::now().into(),
                chrono::offset::Utc::now().into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(Post::Table)
            .columns([
                Post::Title,
                Post::Description,
                Post::CategoryName,
                Post::Created,
                Post::Updated,
            ])
            .values_panic([
                "Title 3".into(),
                "description 3".into(),
                "Category1".into(),
                chrono::offset::Utc::now().into(),
                chrono::offset::Utc::now().into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(Post::Table)
            .columns([
                Post::Title,
                Post::Description,
                Post::CategoryName,
                Post::Created,
                Post::Updated,
            ])
            .values_panic([
                "Title 4".into(),
                "description 4".into(),
                "Category1".into(),
                chrono::offset::Utc::now().into(),
                chrono::offset::Utc::now().into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
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
    CategoryName,
    Created,
    Updated,
}
