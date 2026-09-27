use crate::templates::event_detail::EventDetailTemplate;
use crate::usecase::event::show_event_usecase::{
    ShowEventInput, ShowEventOutput, ShowEventUseCase,
};
use domains_event::event::event_repository::RepositoryError;
use libs_usecase::usecase::usecase::UseCase;
use uuid::Uuid;

pub(crate) struct ShowEventPresentation {
    pub(crate) template: EventDetailTemplate,
}

pub(crate) struct ShowEventPresenter {
    show_event_use_case: ShowEventUseCase,
}

impl ShowEventPresenter {
    pub(crate) fn new(show_event_use_case: ShowEventUseCase) -> Self {
        Self {
            show_event_use_case,
        }
    }

    pub(crate) async fn present(
        &self,
        id: Uuid,
    ) -> Result<ShowEventPresentation, ShowEventPresenterError> {
        let output = self
            .show_event_use_case
            .handle(ShowEventInput::new(id))
            .await
            .map_err(ShowEventPresenterError::UseCase)?;

        match output {
            ShowEventOutput::Shown {
                event,
                participants,
            } => Ok(ShowEventPresentation {
                template: EventDetailTemplate::new(&event, participants),
            }),
            ShowEventOutput::EventNotFound => Err(ShowEventPresenterError::NotFound),
        }
    }
}

pub(crate) enum ShowEventPresenterError {
    NotFound,
    UseCase(RepositoryError),
}
