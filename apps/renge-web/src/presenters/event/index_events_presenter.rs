use crate::templates::event_index::EventIndexTemplate;
use crate::usecase::event::list_events_usecase::{
    EVENT_PAGE_SIZE, ListEventsInput, ListEventsUseCase,
};
use domains_event::event::event_repository::RepositoryError;
use libs_paging::paging::offset_paging::OffsetPaging;
use libs_usecase::usecase::usecase::UseCase;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct EventIndexRequest {
    pub(crate) page: Option<String>,
}

pub(crate) struct EventIndexPresentation {
    pub(crate) template: EventIndexTemplate,
}

pub(crate) struct IndexEventsPresenter {
    list_events_use_case: ListEventsUseCase,
}

pub(crate) enum IndexEventsPresenterError {
    BadRequest,
    UseCase(RepositoryError),
}

impl IndexEventsPresenter {
    pub(crate) fn new(list_events_use_case: ListEventsUseCase) -> Self {
        Self {
            list_events_use_case,
        }
    }

    pub(crate) async fn present(
        &self,
        request: EventIndexRequest,
    ) -> Result<EventIndexPresentation, IndexEventsPresenterError> {
        let page = request
            .page
            .as_deref()
            .unwrap_or("1")
            .parse::<u32>()
            .map_err(|_| IndexEventsPresenterError::BadRequest)?;
        let offset = page
            .checked_sub(1)
            .filter(|_| page > 0)
            .and_then(|page_index| page_index.checked_mul(EVENT_PAGE_SIZE))
            .ok_or(IndexEventsPresenterError::BadRequest)?;
        let output = self
            .list_events_use_case
            .handle(ListEventsInput {
                paging: OffsetPaging::new(EVENT_PAGE_SIZE + 1, offset),
            })
            .await
            .map_err(IndexEventsPresenterError::UseCase)?;

        let has_next = output.events.items.len() > EVENT_PAGE_SIZE as usize;
        let events = output
            .events
            .items
            .into_iter()
            .take(EVENT_PAGE_SIZE as usize)
            .collect();

        Ok(EventIndexPresentation {
            template: EventIndexTemplate::new(events, page, page > 1, has_next),
        })
    }
}
