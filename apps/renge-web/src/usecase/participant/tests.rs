use super::{
    create_participant_usecase::{
        CreateParticipantInput, CreateParticipantOutput, CreateParticipantUseCase,
    },
    delete_participant_usecase::{
        DeleteParticipantInput, DeleteParticipantOutput, DeleteParticipantUseCase,
    },
    test_support::{
        FakeEventRepository, FakeParticipantRepository, FixedIdProvider, sample_event,
        sample_participant,
    },
    update_participant_attendance_usecase::{
        UpdateParticipantAttendanceInput, UpdateParticipantAttendanceOutput,
        UpdateParticipantAttendanceUseCase,
    },
};
use domains_event::{
    event::event_id::EventId,
    participant::{ParticipantAttendance, ParticipantEmail, ParticipantId, ParticipantName},
};
use libs_modeling::identifier::Identifier;
use libs_usecase::usecase::usecase::UseCase;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn create_participant_checks_event_and_starts_pending() {
    let event_id = Uuid::from_u128(1);
    let participant_id = Uuid::from_u128(2);
    let events = Arc::new(FakeEventRepository::new(vec![sample_event(event_id)]));
    let participants = Arc::new(FakeParticipantRepository::default());
    let use_case = CreateParticipantUseCase::new(
        Arc::new(FixedIdProvider(participant_id)),
        events.clone(),
        participants.clone(),
    );

    let output = use_case
        .handle(CreateParticipantInput {
            event_id: EventId::new(event_id),
            name: ParticipantName::try_new("参加者".to_owned()).unwrap(),
            email: ParticipantEmail::try_new("person@example.com".to_owned()).unwrap(),
        })
        .await
        .unwrap();
    let created = match output {
        CreateParticipantOutput::Created { participant } => participant,
        CreateParticipantOutput::EventNotFound => panic!("event should exist"),
    };

    assert_eq!(created.id.value(), participant_id);
    assert_eq!(created.attendance, ParticipantAttendance::Pending);
    assert_eq!(participants.all(), vec![created]);

    let missing = use_case
        .handle(CreateParticipantInput {
            event_id: EventId::new(Uuid::from_u128(99)),
            name: ParticipantName::try_new("別の人".to_owned()).unwrap(),
            email: ParticipantEmail::try_new("other@example.com".to_owned()).unwrap(),
        })
        .await
        .unwrap();
    assert!(matches!(missing, CreateParticipantOutput::EventNotFound));
}

#[tokio::test]
async fn update_attendance_is_scoped_to_the_event() {
    let event_id = Uuid::from_u128(1);
    let participant_id = Uuid::from_u128(2);
    let participants = Arc::new(FakeParticipantRepository::new(vec![sample_participant(
        event_id,
        participant_id,
    )]));
    let use_case = UpdateParticipantAttendanceUseCase::new(participants.clone());

    let output = use_case
        .handle(UpdateParticipantAttendanceInput {
            event_id: EventId::new(event_id),
            participant_id: ParticipantId::new(participant_id),
            attendance: ParticipantAttendance::Attending,
        })
        .await
        .unwrap();
    let updated = match output {
        UpdateParticipantAttendanceOutput::Updated { participant } => participant,
        UpdateParticipantAttendanceOutput::ParticipantNotFound => {
            panic!("participant should exist")
        }
    };
    assert_eq!(updated.attendance, ParticipantAttendance::Attending);

    let missing = use_case
        .handle(UpdateParticipantAttendanceInput {
            event_id: EventId::new(Uuid::from_u128(99)),
            participant_id: ParticipantId::new(participant_id),
            attendance: ParticipantAttendance::Declined,
        })
        .await
        .unwrap();
    assert!(matches!(
        missing,
        UpdateParticipantAttendanceOutput::ParticipantNotFound
    ));
    assert_eq!(
        participants.all()[0].attendance,
        ParticipantAttendance::Attending
    );
}

#[tokio::test]
async fn delete_is_event_scoped_and_missing_rows_succeed() {
    let event_id = Uuid::from_u128(1);
    let participant_id = Uuid::from_u128(2);
    let participants = Arc::new(FakeParticipantRepository::new(vec![sample_participant(
        event_id,
        participant_id,
    )]));
    let use_case = DeleteParticipantUseCase::new(participants.clone());

    let result = use_case
        .handle(DeleteParticipantInput::new(
            Uuid::from_u128(99),
            participant_id,
        ))
        .await
        .unwrap();
    assert!(matches!(result, DeleteParticipantOutput::Deleted));
    assert_eq!(participants.all().len(), 1);

    use_case
        .handle(DeleteParticipantInput::new(event_id, participant_id))
        .await
        .unwrap();
    assert!(participants.all().is_empty());

    use_case
        .handle(DeleteParticipantInput::new(event_id, participant_id))
        .await
        .unwrap();
}
