use crate::event::Event;
use crate::event::event_id::EventId;
use async_trait::async_trait;
use libs_paging::paging::offset_paged::OffsetPaged;
use libs_paging::paging::offset_paging::OffsetPaging;
use std::error::Error;

pub type RepositoryError = Box<dyn Error + Send + Sync>;

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_by_id(&self, id: &EventId) -> Result<Option<Event>, RepositoryError>;

    async fn list(&self, paging: OffsetPaging) -> Result<OffsetPaged<Event>, RepositoryError>;

    async fn insert(&self, event: &Event) -> Result<(), RepositoryError>;

    async fn update(&self, event: &Event) -> Result<(), RepositoryError>;

    async fn delete(&self, id: &EventId) -> Result<(), RepositoryError>;
}
