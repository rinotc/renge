use crate::event::event_id::EventId;
use crate::participant::{ParticipantAttendance, ParticipantEmail, ParticipantId, ParticipantName};
use libs_modeling::id_provider::IdProvider;
use libs_modeling::identifier::Identifier;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Participant {
    pub id: ParticipantId,
    pub event_id: EventId,
    pub name: ParticipantName,
    pub email: ParticipantEmail,
    pub attendance: ParticipantAttendance,
}

impl Participant {
    pub fn create(
        id_provider: &dyn IdProvider<Uuid>,
        event_id: EventId,
        name: ParticipantName,
        email: ParticipantEmail,
    ) -> Self {
        Self {
            id: ParticipantId::generate(id_provider),
            event_id,
            name,
            email,
            attendance: ParticipantAttendance::Pending,
        }
    }

    pub fn new(
        id: ParticipantId,
        event_id: EventId,
        name: ParticipantName,
        email: ParticipantEmail,
        attendance: ParticipantAttendance,
    ) -> Self {
        Self {
            id,
            event_id,
            name,
            email,
            attendance,
        }
    }

    pub fn set_attendance(&mut self, attendance: ParticipantAttendance) {
        self.attendance = attendance;
    }
}

#[cfg(test)]
mod tests {
    use super::Participant;
    use crate::{
        event::event_id::EventId,
        participant::{ParticipantAttendance, ParticipantEmail, ParticipantName},
    };
    use libs_modeling::{id_provider::IdProvider, identifier::Identifier};
    use uuid::Uuid;

    struct FixedIdProvider(Uuid);

    impl IdProvider<Uuid> for FixedIdProvider {
        fn generate(&self) -> Uuid {
            self.0
        }
    }

    #[test]
    fn creation_assigns_an_id_and_pending_attendance() {
        let id = Uuid::from_u128(1);
        let event_id = EventId::new(Uuid::from_u128(2));
        let participant = Participant::create(
            &FixedIdProvider(id),
            event_id.clone(),
            ParticipantName::try_new("参加者".to_owned()).unwrap(),
            ParticipantEmail::try_new("person@example.com".to_owned()).unwrap(),
        );

        assert_eq!(participant.id.value(), id);
        assert_eq!(participant.event_id, event_id);
        assert_eq!(participant.attendance, ParticipantAttendance::Pending);
    }
}
