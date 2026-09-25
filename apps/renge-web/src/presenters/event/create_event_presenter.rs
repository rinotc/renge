use crate::{usecase::event::create_event_usecase::CreateEventUseCase, views::EventCardTemplate};
use chrono::NaiveDateTime;
use domains_event::event::{
    Event,
    event_description::{EventDescription, EventDescriptionError},
    event_location::EventLocation,
    event_repository::RepositoryError,
    event_title::{EventTitle, EventTitleError},
};
use libs_usecase::usecase::usecase::UseCase;
use serde::Deserialize;
use uuid::Uuid;

use crate::usecase::event::create_event_usecase::{CreateEventInput, CreateEventOutput};

#[derive(Deserialize)]
pub(crate) struct CreateEventRequest {
    title: String,
    description: String,
    starts_at: String,
    location: String,
}

pub(crate) struct CreateEventPresentation {
    pub(crate) event_id: Uuid,
    pub(crate) event_card: EventCardTemplate,
}

pub(crate) struct CreateEventPresenter {
    create_event_use_case: CreateEventUseCase,
}

impl CreateEventPresenter {
    pub(crate) fn new(create_event_use_case: CreateEventUseCase) -> Self {
        Self {
            create_event_use_case,
        }
    }

    pub(crate) async fn present(
        &self,
        request: CreateEventRequest,
    ) -> Result<CreateEventPresentation, CreateEventPresenterError> {
        let input = to_use_case_input(request)?;
        let output = self
            .create_event_use_case
            .handle(input)
            .await
            .map_err(CreateEventPresenterError::UseCase)?;

        match output {
            CreateEventOutput::Created { event } => Ok(presentation_for(event)),
        }
    }
}

pub(crate) enum CreateEventPresenterError {
    BadRequest(String),
    UseCase(RepositoryError),
}

fn presentation_for(event: Event) -> CreateEventPresentation {
    CreateEventPresentation {
        event_id: event.id.value(),
        event_card: EventCardTemplate::from_domain(&event),
    }
}

fn to_use_case_input(
    request: CreateEventRequest,
) -> Result<CreateEventInput, CreateEventPresenterError> {
    let title = required(&request.title, "イベント名")?;
    let title = EventTitle::try_new(title).map_err(event_title_error)?;

    let description = optional(&request.description)
        .map(|description| EventDescription::try_new(description).map_err(event_description_error))
        .transpose()?;

    let starts_at = NaiveDateTime::parse_from_str(&request.starts_at, "%Y-%m-%dT%H:%M")
        .map(|date| date.and_utc().fixed_offset())
        .map_err(|_| {
            CreateEventPresenterError::BadRequest("開催日時を入力してください。".into())
        })?;

    let location = optional(&request.location).map(EventLocation::new);

    Ok(CreateEventInput {
        title,
        description,
        starts_at,
        location,
    })
}

fn required(value: &str, name: &str) -> Result<String, CreateEventPresenterError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(CreateEventPresenterError::BadRequest(format!(
            "{name}を入力してください。"
        )));
    }
    Ok(value.to_owned())
}

fn optional(value: &str) -> Option<String> {
    (!value.trim().is_empty()).then(|| value.trim().to_owned())
}

fn event_title_error(error: EventTitleError) -> CreateEventPresenterError {
    let message = match error {
        EventTitleError::BlankNotAllowed => "イベント名を入力してください。",
        EventTitleError::TooLong => "イベント名は100文字以内で入力してください。",
    };
    CreateEventPresenterError::BadRequest(message.into())
}

fn event_description_error(error: EventDescriptionError) -> CreateEventPresenterError {
    let message = match error {
        EventDescriptionError::BlankNotAllowed => "説明を入力してください。",
        EventDescriptionError::TooLong => "説明は1000文字以内で入力してください。",
    };
    CreateEventPresenterError::BadRequest(message.into())
}
