use async_trait::async_trait;
use domains_event::event::event_repository::{EventRepository, RepositoryError};
use libs_paging::paging::offset_paged::OffsetPaged;
use libs_paging::paging::offset_paging::OffsetPaging;
use std::sync::Arc;

pub const EVENT_PAGE_SIZE: u32 = 20;

pub struct ListEventsUseCase {
    event_repository: Arc<dyn EventRepository>,
}

impl ListEventsUseCase {
    pub fn new(event_repository: Arc<dyn EventRepository>) -> Self {
        Self { event_repository }
    }
}

#[async_trait]
impl libs_usecase::usecase::usecase::UseCase for ListEventsUseCase {
    type Input = ListEventsInput;
    type Output = ListEventsOutput;
    type Error = RepositoryError;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let events = self.event_repository.list(input.paging).await?;
        Ok(ListEventsOutput { events })
    }
}

pub struct ListEventsInput {
    pub paging: OffsetPaging,
}

pub struct ListEventsOutput {
    pub events: OffsetPaged<domains_event::event::Event>,
}
