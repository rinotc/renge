pub mod event {
    use chrono::{DateTime, Utc};
    use sea_orm::entity::prelude::*;
    use uuid::Uuid;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "events")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub title: String,
        pub description: Option<String>,
        pub starts_at: DateTime<Utc>,
        pub location: Option<String>,
        pub created_at: DateTime<Utc>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::participant::Entity")]
        Participants,
    }
    impl ActiveModelBehavior for ActiveModel {}
}
pub mod participant {
    use chrono::{DateTime, Utc};
    use sea_orm::entity::prelude::*;
    use uuid::Uuid;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "participants")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub event_id: Uuid,
        pub name: String,
        pub email: String,
        pub attendance: String,
        pub created_at: DateTime<Utc>,
    }
    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::event::Entity",
            from = "Column::EventId",
            to = "super::event::Column::Id",
            on_delete = "Cascade"
        )]
        Event,
    }
    impl Related<super::event::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Event.def()
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}
