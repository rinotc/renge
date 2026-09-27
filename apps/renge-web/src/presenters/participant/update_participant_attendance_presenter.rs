use crate::{
    templates::participant_row::ParticipantRowTemplate,
    usecase::participant::update_participant_attendance_usecase::{
        UpdateParticipantAttendanceInput, UpdateParticipantAttendanceOutput,
        UpdateParticipantAttendanceUseCase,
    },
};
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::RepositoryError;
use domains_event::participant::{ParticipantAttendance, ParticipantId};
use libs_modeling::identifier::Identifier;
use libs_usecase::usecase::usecase::UseCase;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct UpdateParticipantAttendanceRequest {
    attendance: String,
}

pub(crate) struct UpdateParticipantAttendancePresentation {
    pub(crate) participant_row: ParticipantRowTemplate,
}

pub(crate) struct UpdateParticipantAttendancePresenter {
    update_participant_attendance_use_case: UpdateParticipantAttendanceUseCase,
}

impl UpdateParticipantAttendancePresenter {
    pub(crate) fn new(
        update_participant_attendance_use_case: UpdateParticipantAttendanceUseCase,
    ) -> Self {
        Self {
            update_participant_attendance_use_case,
        }
    }

    pub(crate) async fn present(
        &self,
        event_id: Uuid,
        participant_id: Uuid,
        request: UpdateParticipantAttendanceRequest,
    ) -> Result<UpdateParticipantAttendancePresentation, UpdateParticipantAttendancePresenterError>
    {
        let attendance = ParticipantAttendance::parse(&request.attendance)
            .ok_or(UpdateParticipantAttendancePresenterError::BadRequest)?;
        let output = self
            .update_participant_attendance_use_case
            .handle(UpdateParticipantAttendanceInput {
                event_id: EventId::new(event_id),
                participant_id: ParticipantId::new(participant_id),
                attendance,
            })
            .await
            .map_err(UpdateParticipantAttendancePresenterError::UseCase)?;

        let participant = match output {
            UpdateParticipantAttendanceOutput::Updated { participant } => participant,
            UpdateParticipantAttendanceOutput::ParticipantNotFound => {
                return Err(UpdateParticipantAttendancePresenterError::NotFound);
            }
        };

        Ok(UpdateParticipantAttendancePresentation {
            participant_row: ParticipantRowTemplate::new(&participant),
        })
    }
}

pub(crate) enum UpdateParticipantAttendancePresenterError {
    BadRequest,
    NotFound,
    UseCase(RepositoryError),
}

#[cfg(test)]
mod tests {
    use super::{
        UpdateParticipantAttendancePresenter, UpdateParticipantAttendancePresenterError,
        UpdateParticipantAttendanceRequest,
    };
    use crate::usecase::participant::{
        test_support::FakeParticipantRepository,
        update_participant_attendance_usecase::UpdateParticipantAttendanceUseCase,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn rejects_unknown_attendance_values() {
        let presenter = UpdateParticipantAttendancePresenter::new(
            UpdateParticipantAttendanceUseCase::new(Arc::new(FakeParticipantRepository::default())),
        );

        let result = presenter
            .present(
                Uuid::from_u128(1),
                Uuid::from_u128(2),
                UpdateParticipantAttendanceRequest {
                    attendance: "unknown".to_owned(),
                },
            )
            .await;

        assert!(matches!(
            result,
            Err(UpdateParticipantAttendancePresenterError::BadRequest)
        ));
    }

    #[tokio::test]
    async fn reports_a_participant_outside_the_event_as_not_found() {
        let presenter = UpdateParticipantAttendancePresenter::new(
            UpdateParticipantAttendanceUseCase::new(Arc::new(FakeParticipantRepository::default())),
        );

        let result = presenter
            .present(
                Uuid::from_u128(1),
                Uuid::from_u128(2),
                UpdateParticipantAttendanceRequest {
                    attendance: "attending".to_owned(),
                },
            )
            .await;

        assert!(matches!(
            result,
            Err(UpdateParticipantAttendancePresenterError::NotFound)
        ));
    }
}
