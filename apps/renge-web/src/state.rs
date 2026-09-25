use crate::{
    presenters::event::create_event_presenter::CreateEventPresenter,
    usecase::event::create_event_usecase::CreateEventUseCase,
};
use adapters_event::event::postgres_event_repository::PostgresEventRepository;
use domains_event::event::event_repository::EventRepository;
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
}

impl AppState {
    pub async fn from_env() -> Result<Self, AppStateError> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| AppStateError::DatabaseUrlNotFound)?;
        let dbc = Database::connect(database_url)
            .await
            .map_err(|e| AppStateError::DatabaseConnectionError(e))?;

        let clock: Arc<dyn Clock> = Arc::new(SystemClock::new());
        let id_provider: Arc<dyn IdProvider<Uuid>> = Arc::new(UuidIdProvider::new(clock.clone()));
        let event_repository: Arc<dyn EventRepository> =
            Arc::new(PostgresEventRepository::new(dbc.clone()));
        let create_event_usecase =
            CreateEventUseCase::new(id_provider.clone(), event_repository.clone());

        Ok(Self {
            dbc,
            create_event_presenter: Arc::new(CreateEventPresenter::new(create_event_usecase)),
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
