use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Events::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Events::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Events::Title).string().not_null())
                    .col(ColumnDef::new(Events::Description).text())
                    .col(
                        ColumnDef::new(Events::StartsAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Events::Location).string())
                    .col(
                        ColumnDef::new(Events::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Participants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Participants::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Participants::EventId).uuid().not_null())
                    .col(ColumnDef::new(Participants::Name).string().not_null())
                    .col(ColumnDef::new(Participants::Email).string().not_null())
                    .col(
                        ColumnDef::new(Participants::Attendance)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(Participants::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-participants-event-id")
                            .from(Participants::Table, Participants::EventId)
                            .to(Events::Table, Events::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-participants-event-id")
                    .table(Participants::Table)
                    .col(Participants::EventId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Participants::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Events::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Events {
    Table,
    Id,
    Title,
    Description,
    StartsAt,
    Location,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Participants {
    Table,
    Id,
    EventId,
    Name,
    Email,
    Attendance,
    CreatedAt,
}
