use crate::usecase::event::delete_event_usecase::{DeleteEventInput, DeleteEventUseCase};
use domains_event::event::event_repository::RepositoryError;
use libs_usecase::usecase::usecase::UseCase;
use uuid::Uuid;

pub(crate) struct DeleteEventPresenter {
    delete_event_use_case: DeleteEventUseCase,
}

impl DeleteEventPresenter {
    pub(crate) fn new(delete_event_use_case: DeleteEventUseCase) -> Self {
        Self {
            delete_event_use_case,
        }
    }

    pub(crate) async fn present(&self, id: Uuid) -> Result<(), DeleteEventPresenterError> {
        self.delete_event_use_case
            .handle(DeleteEventInput::new(id))
            .await
            .map_err(DeleteEventPresenterError::UseCase)?;
        Ok(())
    }
}

pub(crate) enum DeleteEventPresenterError {
    UseCase(RepositoryError),
}
