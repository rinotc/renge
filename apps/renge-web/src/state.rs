use crate::{
    presenters::event::{
        create_event_presenter::CreateEventPresenter, delete_event_presenter::DeleteEventPresenter,
        index_events_presenter::IndexEventsPresenter, show_event_presenter::ShowEventPresenter,
    },
    usecase::event::{
        create_event_usecase::CreateEventUseCase, delete_event_usecase::DeleteEventUseCase,
        list_events_usecase::ListEventsUseCase, show_event_usecase::ShowEventUseCase,
    },
};
use adapters_event::{
    event::postgres_event_repository::PostgresEventRepository,
    participant::postgres_participant_repository::PostgresParticipantRepository,
};
use domains_event::event::event_repository::EventRepository;
use domains_event::participant::ParticipantRepository;
use libs_clock::clock::{Clock, SystemClock};
use libs_modeling::id_provider::{IdProvider, UuidIdProvider};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::error::Error;
use std::sync::Arc;
use std::{env, fmt};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub dbc: DatabaseConnection,
    pub(crate) create_event_presenter: Arc<CreateEventPresenter>,
    pub(crate) index_events_presenter: Arc<IndexEventsPresenter>,
    pub(crate) show_event_presenter: Arc<ShowEventPresenter>,
    pub(crate) delete_event_presenter: Arc<DeleteEventPresenter>,
}

impl AppState {
    pub async fn from_env() -> Result<Self, AppStateError> {
        // Database
        let database_url =
            env::var("DATABASE_URL").map_err(|_| AppStateError::DatabaseUrlNotFound)?;
        let dbc = Database::connect(database_url)
            .await
            .map_err(|e| AppStateError::DatabaseConnectionError(e))?;

        // utils
        let clock: Arc<dyn Clock> = Arc::new(SystemClock::new());
        let id_provider: Arc<dyn IdProvider<Uuid>> = Arc::new(UuidIdProvider::new(clock.clone()));

        // Repository, Service
        let event_repository: Arc<dyn EventRepository> =
            Arc::new(PostgresEventRepository::new(dbc.clone()));
        let participant_repository: Arc<dyn ParticipantRepository> =
            Arc::new(PostgresParticipantRepository::new(dbc.clone()));
        let create_event_usecase =
            CreateEventUseCase::new(id_provider.clone(), event_repository.clone());

        // UseCase
        let list_events_usecase = ListEventsUseCase::new(event_repository.clone());
        let show_event_usecase =
            ShowEventUseCase::new(event_repository.clone(), participant_repository);
        let delete_event_usecase = DeleteEventUseCase::new(event_repository);

        Ok(Self {
            dbc,
            create_event_presenter: Arc::new(CreateEventPresenter::new(create_event_usecase)),
            index_events_presenter: Arc::new(IndexEventsPresenter::new(list_events_usecase)),
            show_event_presenter: Arc::new(ShowEventPresenter::new(show_event_usecase)),
            delete_event_presenter: Arc::new(DeleteEventPresenter::new(delete_event_usecase)),
        })
    }
}

#[derive(Debug)]
pub enum AppStateError {
    DatabaseUrlNotFound,
    DatabaseConnectionError(DbErr),
}

impl fmt::Display for AppStateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppStateError::DatabaseUrlNotFound => write!(
                f,
                "DATABASE_URL が未設定です。.env.example を参考に設定してください。"
            ),
            AppStateError::DatabaseConnectionError(e) => {
                write!(f, "データベース接続に失敗しました: {e}")
            }
        }
    }
}

impl Error for AppStateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::DatabaseUrlNotFound => None,
            Self::DatabaseConnectionError(e) => Some(e),
        }
    }
}
