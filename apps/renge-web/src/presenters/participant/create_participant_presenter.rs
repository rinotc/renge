use crate::{
    templates::participant_row::ParticipantRowTemplate,
    usecase::participant::create_participant_usecase::{
        CreateParticipantInput, CreateParticipantOutput, CreateParticipantUseCase,
    },
};
use domains_event::event::event_id::EventId;
use domains_event::event::event_repository::RepositoryError;
use domains_event::participant::{ParticipantEmail, ParticipantEmailError, ParticipantName};
use libs_modeling::identifier::Identifier;
use libs_usecase::usecase::usecase::UseCase;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub(crate) struct CreateParticipantRequest {
    name: String,
    email: String,
}

pub(crate) struct CreateParticipantPresentation {
    pub(crate) event_id: Uuid,
    pub(crate) participant_row: ParticipantRowTemplate,
}

pub(crate) struct CreateParticipantPresenter {
    create_participant_use_case: CreateParticipantUseCase,
}

impl CreateParticipantPresenter {
    pub(crate) fn new(create_participant_use_case: CreateParticipantUseCase) -> Self {
        Self {
            create_participant_use_case,
        }
    }

    pub(crate) async fn present(
        &self,
        event_id: Uuid,
        request: CreateParticipantRequest,
    ) -> Result<CreateParticipantPresentation, CreateParticipantPresenterError> {
        let name = ParticipantName::try_new(required(&request.name, "氏名")?).map_err(|_| {
            CreateParticipantPresenterError::BadRequest("氏名を入力してください。".into())
        })?;
        let email = ParticipantEmail::try_new(required(&request.email, "メールアドレス")?)
            .map_err(email_error)?;

        let output = self
            .create_participant_use_case
            .handle(CreateParticipantInput {
                event_id: EventId::new(event_id),
                name,
                email,
            })
            .await
            .map_err(CreateParticipantPresenterError::UseCase)?;

        let participant = match output {
            CreateParticipantOutput::Created { participant } => participant,
            CreateParticipantOutput::EventNotFound => {
                return Err(CreateParticipantPresenterError::NotFound);
            }
        };

        Ok(CreateParticipantPresentation {
            event_id,
            participant_row: ParticipantRowTemplate::new(&participant),
        })
    }
}

pub(crate) enum CreateParticipantPresenterError {
    BadRequest(String),
    NotFound,
    UseCase(RepositoryError),
}

fn required(value: &str, name: &str) -> Result<String, CreateParticipantPresenterError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(CreateParticipantPresenterError::BadRequest(format!(
            "{name}を入力してください。"
        )));
    }
    Ok(value.to_owned())
}

fn email_error(error: ParticipantEmailError) -> CreateParticipantPresenterError {
    let message = match error {
        ParticipantEmailError::BlankNotAllowed => "メールアドレスを入力してください。",
        ParticipantEmailError::InvalidFormat => "メールアドレスの形式が正しくありません。",
    };
    CreateParticipantPresenterError::BadRequest(message.into())
}

#[cfg(test)]
mod tests {
    use super::{
        CreateParticipantPresenter, CreateParticipantPresenterError, CreateParticipantRequest,
    };
    use crate::usecase::participant::{
        create_participant_usecase::CreateParticipantUseCase,
        test_support::{
            FakeEventRepository, FakeParticipantRepository, FixedIdProvider, sample_event,
        },
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[tokio::test]
    async fn rejects_invalid_email_before_calling_the_use_case() {
        let event_id = Uuid::from_u128(1);
        let presenter = CreateParticipantPresenter::new(CreateParticipantUseCase::new(
            Arc::new(FixedIdProvider(Uuid::from_u128(2))),
            Arc::new(FakeEventRepository::new(vec![sample_event(event_id)])),
            Arc::new(FakeParticipantRepository::default()),
        ));

        let result = presenter
            .present(
                event_id,
                CreateParticipantRequest {
                    name: "参加者".to_owned(),
                    email: "not-an-email".to_owned(),
                },
            )
            .await;

        assert!(matches!(
            result,
            Err(CreateParticipantPresenterError::BadRequest(message))
                if message == "メールアドレスの形式が正しくありません。"
        ));
    }

    #[tokio::test]
    async fn reports_missing_event_as_not_found() {
        let presenter = CreateParticipantPresenter::new(CreateParticipantUseCase::new(
            Arc::new(FixedIdProvider(Uuid::from_u128(2))),
            Arc::new(FakeEventRepository::default()),
            Arc::new(FakeParticipantRepository::default()),
        ));

        let result = presenter
            .present(
                Uuid::from_u128(1),
                CreateParticipantRequest {
                    name: "参加者".to_owned(),
                    email: "person@example.com".to_owned(),
                },
            )
            .await;

        assert!(matches!(
            result,
            Err(CreateParticipantPresenterError::NotFound)
        ));
    }
}
