use async_trait::async_trait;
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::RepositoryError;
use domains_event::participant::{
    Participant, ParticipantAttendance, ParticipantId, ParticipantRepository,
};
use libs_usecase::usecase::usecase::UseCase;
use std::sync::Arc;

pub struct UpdateParticipantAttendanceUseCase {
    participant_repository: Arc<dyn ParticipantRepository>,
}

impl UpdateParticipantAttendanceUseCase {
    pub fn new(participant_repository: Arc<dyn ParticipantRepository>) -> Self {
        Self {
            participant_repository,
        }
    }
}

#[async_trait]
impl UseCase for UpdateParticipantAttendanceUseCase {
    type Input = UpdateParticipantAttendanceInput;
    type Output = UpdateParticipantAttendanceOutput;
    type Error = RepositoryError;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let Some(mut participant) = self
            .participant_repository
            .find_by_event_and_id(&input.event_id, &input.participant_id)
            .await?
        else {
            return Ok(UpdateParticipantAttendanceOutput::ParticipantNotFound);
        };

        participant.set_attendance(input.attendance);
        self.participant_repository.update(&participant).await?;
        Ok(UpdateParticipantAttendanceOutput::Updated { participant })
    }
}

pub struct UpdateParticipantAttendanceInput {
    pub event_id: EventId,
    pub participant_id: ParticipantId,
    pub attendance: ParticipantAttendance,
}

pub enum UpdateParticipantAttendanceOutput {
    Updated { participant: Participant },
    ParticipantNotFound,
}
