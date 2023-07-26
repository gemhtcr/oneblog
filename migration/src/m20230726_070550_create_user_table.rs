use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::UserId)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .to_owned(),
            )
            .await?;

		let db = manager.get_connection();
		// Construct a `Statement` if the SQL contains value bindings
		//let stmt = sea_orm::Statement::from_string(
		//	manager.get_database_backend(),
		//	r#"INSERT INTO users (user_id, username, password_hash) VALUES (
        //        ddf8994f-d522-4659-8d02-c1d479057be6,
        //        admin,
        //        $argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8
        //    )"#
		//);
		let stmt = sea_orm::Statement::from_sql_and_values(
			manager.get_database_backend(),
			r#"INSERT INTO `users` (`user_id`, `username`, `password_hash`) VALUES(?, ?, ?)"#,
			[
                "ddf8994f-d522-4659-8d02-c1d479057be6".into(),
                "admin".into(),
                "$argon2id$v=19$m=15000,t=2,p=1$OEx/rcq+3ts//WUDzGNl2g$Am8UFBA4w5NJEmAtquGvBmAlu92q/VQcaoL5AyJPfc8".into()
			],
		);
		db.execute(stmt).await?;


		Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Users {
    Table,
    UserId,
    Username,
    PasswordHash,
}
