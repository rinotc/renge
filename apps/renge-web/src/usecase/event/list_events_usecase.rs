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
        Ok(ListEventsOutput::Listed { events })
    }
}

pub struct ListEventsInput {
    pub paging: OffsetPaging,
}

pub enum ListEventsOutput {
    Listed {
        events: OffsetPaged<domains_event::event::Event>,
    },
}

#[cfg(test)]
mod tests {
    use super::{ListEventsInput, ListEventsOutput, ListEventsUseCase};
    use crate::usecase::participant::test_support::FakeEventRepository;
    use libs_paging::paging::offset_paging::OffsetPaging;
    use libs_usecase::usecase::usecase::UseCase;
    use std::sync::Arc;

    #[tokio::test]
    async fn returns_listed_output() {
        let use_case = ListEventsUseCase::new(Arc::new(FakeEventRepository::default()));

        let output = use_case
            .handle(ListEventsInput {
                paging: OffsetPaging::new(20, 0),
            })
            .await
            .unwrap();

        match output {
            ListEventsOutput::Listed { events } => assert!(events.items.is_empty()),
        }
    }
}
