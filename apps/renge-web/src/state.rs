use sea_orm::{Database, DatabaseConnection, DbErr};
use std::error::Error;
use std::{env, fmt};

#[derive(Clone)]
pub struct AppState {
    pub dbc: DatabaseConnection,
}

impl AppState {
    pub async fn from_env() -> Result<Self, AppStateError> {
        let database_url =
            env::var("DATABASE_URL").map_err(|_| AppStateError::DatabaseUrlNotFound)?;
        let dbc = Database::connect(database_url)
            .await
            .map_err(|e| AppStateError::DatabaseConnectionError(e))?;
        Ok(Self { dbc })
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
