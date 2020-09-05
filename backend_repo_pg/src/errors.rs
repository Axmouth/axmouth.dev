#[derive(Debug, Clone)]
pub enum PgRepoErrorType {
    Unknown,
}

#[derive(Debug, Clone)]
pub struct PgRepoError {
    pub error_message: String,
    pub error_type: PgRepoErrorType,
}

impl std::error::Error for PgRepoError {}
impl std::fmt::Display for PgRepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl From<diesel::result::Error> for PgRepoError {
    fn from(error: diesel::result::Error) -> PgRepoError {
        PgRepoError {
            error_message: error.to_string(),
            error_type: PgRepoErrorType::Unknown,
        }
    }
}

impl From<r2d2::Error> for PgRepoError {
    fn from(error: r2d2::Error) -> PgRepoError {
        PgRepoError {
            error_message: error.to_string(),
            error_type: PgRepoErrorType::Unknown,
        }
    }
}
