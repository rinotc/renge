use async_trait::async_trait;
use domains_event::event::Event;
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::{EventRepository, RepositoryError};
use domains_event::participant::{Participant, ParticipantRepository};
use libs_modeling::identifier::Identifier;
use std::sync::Arc;
use uuid::Uuid;

pub struct ShowEventUseCase {
    event_repository: Arc<dyn EventRepository>,
    participant_repository: Arc<dyn ParticipantRepository>,
}

impl ShowEventUseCase {
    pub fn new(
        event_repository: Arc<dyn EventRepository>,
        participant_repository: Arc<dyn ParticipantRepository>,
    ) -> Self {
        Self {
            event_repository,
            participant_repository,
        }
    }
}

#[async_trait]
impl libs_usecase::usecase::usecase::UseCase for ShowEventUseCase {
    type Input = ShowEventInput;
    type Output = ShowEventOutput;
    type Error = RepositoryError;

    async fn handle(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let Some(event) = self.event_repository.find_by_id(&input.event_id).await? else {
            return Ok(ShowEventOutput::EventNotFound);
        };
        let participants = self
            .participant_repository
            .list_by_event_id(&input.event_id)
            .await?;

        Ok(ShowEventOutput::Shown {
            event,
            participants,
        })
    }
}

pub struct ShowEventInput {
    pub event_id: EventId,
}

impl ShowEventInput {
    pub fn new(event_id: Uuid) -> Self {
        Self {
            event_id: EventId::new(event_id),
        }
    }
}

pub enum ShowEventOutput {
    Shown {
        event: Event,
        participants: Vec<Participant>,
    },
    EventNotFound,
}
#[cfg(test)]
mod tests {
    use super::{ShowEventInput, ShowEventOutput, ShowEventUseCase};
    use crate::usecase::participant::test_support::{
        FakeEventRepository, FakeParticipantRepository, sample_event, sample_participant,
    };
    use libs_usecase::usecase::usecase::UseCase;
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn returns_shown_output_with_participants() {
        let event_id = Uuid::from_u128(1);
        let use_case = ShowEventUseCase::new(
            Arc::new(FakeEventRepository::new(vec![sample_event(event_id)])),
            Arc::new(FakeParticipantRepository::new(vec![sample_participant(
                event_id,
                Uuid::from_u128(2),
            )])),
        );

        match use_case
            .handle(ShowEventInput::new(event_id))
            .await
            .unwrap()
        {
            ShowEventOutput::Shown {
                event,
                participants,
            } => {
                assert_eq!(event.id.value(), event_id);
                assert_eq!(participants.len(), 1);
            }
            ShowEventOutput::EventNotFound => panic!("event should exist"),
        }
    }

    #[tokio::test]
    async fn returns_event_not_found_output() {
        let use_case = ShowEventUseCase::new(
            Arc::new(FakeEventRepository::default()),
            Arc::new(FakeParticipantRepository::default()),
        );

        let output = use_case
            .handle(ShowEventInput::new(Uuid::from_u128(99)))
            .await
            .unwrap();

        assert!(matches!(output, ShowEventOutput::EventNotFound));
    }
}
