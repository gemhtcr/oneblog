use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Category::Name)
                            .string()
                            .unique_key()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Category::Created).timestamp().not_null())
                    .col(ColumnDef::new(Category::Updated).timestamp().not_null())
                    .to_owned(),
            )
            .await?;
        let insert = Query::insert()
            .into_table(Category::Table)
            .columns([Category::Name, Category::Created, Category::Updated])
            .values_panic([
                "Category1".into(),
                chrono::offset::Utc::now().into(),
                chrono::offset::Utc::now().into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await?;

        let insert = Query::insert()
            .into_table(Category::Table)
            .columns([Category::Name, Category::Created, Category::Updated])
            .values_panic([
                "Category2".into(),
                chrono::offset::Utc::now().into(),
                chrono::offset::Utc::now().into(),
            ])
            .to_owned();
        manager.exec_stmt(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        //todo!();

        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Category {
    Table,
    Id,
    Name,
    Created,
    Updated,
}
