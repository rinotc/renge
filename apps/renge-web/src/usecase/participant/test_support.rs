use async_trait::async_trait;
use domains_event::{
    event::{
        Event,
        event_description::EventDescription,
        event_id::EventId,
        event_location::EventLocation,
        event_repository::{EventRepository, RepositoryError},
        event_title::EventTitle,
    },
    participant::{Participant, ParticipantId, ParticipantRepository},
};
use libs_modeling::{id_provider::IdProvider, identifier::Identifier};
use libs_paging::paging::{offset_paged::OffsetPaged, offset_paging::OffsetPaging};
use std::sync::Mutex;
use uuid::Uuid;

pub(crate) struct FixedIdProvider(pub(crate) Uuid);

impl IdProvider<Uuid> for FixedIdProvider {
    fn generate(&self) -> Uuid {
        self.0
    }
}

#[derive(Default)]
pub(crate) struct FakeEventRepository {
    events: Mutex<Vec<Event>>,
}

impl FakeEventRepository {
    pub(crate) fn new(events: Vec<Event>) -> Self {
        Self {
            events: Mutex::new(events),
        }
    }
}

#[async_trait]
impl EventRepository for FakeEventRepository {
    async fn find_by_id(&self, id: &EventId) -> Result<Option<Event>, RepositoryError> {
        Ok(self
            .events
            .lock()
            .unwrap()
            .iter()
            .find(|event| &event.id == id)
            .cloned())
    }

    async fn list(&self, paging: OffsetPaging) -> Result<OffsetPaged<Event>, RepositoryError> {
        Ok(OffsetPaged::new(Vec::new(), paging))
    }

    async fn insert(&self, event: &Event) -> Result<(), RepositoryError> {
        self.events.lock().unwrap().push(event.clone());
        Ok(())
    }

    async fn update(&self, event: &Event) -> Result<(), RepositoryError> {
        let mut events = self.events.lock().unwrap();
        if let Some(existing) = events.iter_mut().find(|existing| existing.id == event.id) {
            *existing = event.clone();
        }
        Ok(())
    }

    async fn delete(&self, id: &EventId) -> Result<(), RepositoryError> {
        self.events.lock().unwrap().retain(|event| &event.id != id);
        Ok(())
    }
}

#[derive(Default)]
pub(crate) struct FakeParticipantRepository {
    participants: Mutex<Vec<Participant>>,
}

impl FakeParticipantRepository {
    pub(crate) fn new(participants: Vec<Participant>) -> Self {
        Self {
            participants: Mutex::new(participants),
        }
    }

    pub(crate) fn all(&self) -> Vec<Participant> {
        self.participants.lock().unwrap().clone()
    }
}

#[async_trait]
impl ParticipantRepository for FakeParticipantRepository {
    async fn list_by_event_id(
        &self,
        event_id: &EventId,
    ) -> Result<Vec<Participant>, RepositoryError> {
        Ok(self
            .participants
            .lock()
            .unwrap()
            .iter()
            .filter(|participant| &participant.event_id == event_id)
            .cloned()
            .collect())
    }

    async fn insert(&self, participant: &Participant) -> Result<(), RepositoryError> {
        self.participants.lock().unwrap().push(participant.clone());
        Ok(())
    }

    async fn find_by_event_and_id(
        &self,
        event_id: &EventId,
        participant_id: &ParticipantId,
    ) -> Result<Option<Participant>, RepositoryError> {
        Ok(self
            .participants
            .lock()
            .unwrap()
            .iter()
            .find(|participant| {
                &participant.event_id == event_id && &participant.id == participant_id
            })
            .cloned())
    }

    async fn update(&self, participant: &Participant) -> Result<(), RepositoryError> {
        if let Some(existing) = self
            .participants
            .lock()
            .unwrap()
            .iter_mut()
            .find(|existing| {
                existing.id == participant.id && existing.event_id == participant.event_id
            })
        {
            *existing = participant.clone();
        }
        Ok(())
    }

    async fn delete_by_event_and_id(
        &self,
        event_id: &EventId,
        participant_id: &ParticipantId,
    ) -> Result<(), RepositoryError> {
        self.participants.lock().unwrap().retain(|participant| {
            &participant.event_id != event_id || &participant.id != participant_id
        });
        Ok(())
    }
}

pub(crate) fn sample_event(event_id: Uuid) -> Event {
    Event::new(
        EventId::new(event_id),
        EventTitle::try_new("テストイベント".to_owned()).unwrap(),
        Some(EventDescription::try_new("説明".to_owned()).unwrap()),
        chrono::DateTime::parse_from_rfc3339("2026-09-27T10:00:00+00:00").unwrap(),
        Some(EventLocation::new("会場".to_owned())),
    )
}

pub(crate) fn sample_participant(event_id: Uuid, participant_id: Uuid) -> Participant {
    use domains_event::participant::{ParticipantAttendance, ParticipantName};
    use libs_shared_primitive::email::Email;

    Participant::new(
        ParticipantId::new(participant_id),
        EventId::new(event_id),
        ParticipantName::try_new("参加者".to_owned()).unwrap(),
        Email::try_new("person@example.com".to_owned()).unwrap(),
        ParticipantAttendance::Pending,
    )
}
