use crate::usecase::participant::delete_participant_usecase::{
    DeleteParticipantInput, DeleteParticipantOutput, DeleteParticipantUseCase,
};
use domains_event::event::event_repository::RepositoryError;
use libs_usecase::usecase::usecase::UseCase;
use uuid::Uuid;

pub(crate) struct DeleteParticipantPresenter {
    delete_participant_use_case: DeleteParticipantUseCase,
}

impl DeleteParticipantPresenter {
    pub(crate) fn new(delete_participant_use_case: DeleteParticipantUseCase) -> Self {
        Self {
            delete_participant_use_case,
        }
    }

    pub(crate) async fn present(
        &self,
        event_id: Uuid,
        participant_id: Uuid,
    ) -> Result<(), DeleteParticipantPresenterError> {
        let output = self
            .delete_participant_use_case
            .handle(DeleteParticipantInput::new(event_id, participant_id))
            .await
            .map_err(DeleteParticipantPresenterError::UseCase)?;
        match output {
            DeleteParticipantOutput::Deleted => Ok(()),
        }
    }
}

pub(crate) enum DeleteParticipantPresenterError {
    UseCase(RepositoryError),
}
