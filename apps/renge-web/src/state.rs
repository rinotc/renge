use crate::{
    presenters::event::create_event_presenter::CreateEventPresenter,
    usecase::event::create_event_usecase::CreateEventUseCase,
};
use adapters_event::event::postgres_event_repository::PostgresEventRepository;
use libs_clock::clock::SystemClock;
use libs_modeling::id_provider::UuidIdProvider;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::error::Error;
use std::sync::Arc;
use std::{env, fmt};

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
        let event_repository = PostgresEventRepository::new(dbc.clone());
        let create_event_use_case = CreateEventUseCase::new(
            Box::new(UuidIdProvider::new(Box::new(SystemClock::new()))),
            Box::new(event_repository),
        );

        Ok(Self {
            dbc,
            create_event_presenter: Arc::new(CreateEventPresenter::new(create_event_use_case)),
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
