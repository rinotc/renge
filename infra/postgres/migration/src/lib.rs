pub use sea_orm_migration::prelude::*;

mod m20260906_000001_create_events_and_participants;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20260906_000001_create_events_and_participants::Migration,
        )]
    }
}
