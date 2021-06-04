use backend_repo_pg::errors::PgRepoError;
use warp::reject;

// FileUploadError
#[derive(Debug, Clone)]
pub struct FileUploadError {
    err_message: String,
}

impl warp::reject::Reject for FileUploadError {}

impl FileUploadError {
    pub fn new(err_message: String) -> Self {
        Self { err_message }
    }
}

impl ToString for FileUploadError {
    fn to_string(&self) -> String {
        return self.err_message.clone();
    }
}

impl From<warp::Error> for FileUploadError {
    fn from(error: warp::Error) -> FileUploadError {
        FileUploadError {
            err_message: error.to_string(),
        }
    }
}

impl From<std::io::Error> for FileUploadError {
    fn from(error: std::io::Error) -> FileUploadError {
        FileUploadError {
            err_message: error.to_string(),
        }
    }
}

// EmailError
#[derive(Debug)]
pub struct EmailError {
    err: String,
}

impl reject::Reject for EmailError {}

impl ToString for EmailError {
    fn to_string(&self) -> String {
        return self.err.clone();
    }
}

impl From<lettre_email::error::Error> for EmailError {
    fn from(error: lettre_email::error::Error) -> EmailError {
        EmailError {
            err: error.to_string(),
        }
    }
}

impl From<lettre::smtp::error::Error> for EmailError {
    fn from(error: lettre::smtp::error::Error) -> EmailError {
        EmailError {
            err: error.to_string(),
        }
    }
}

impl From<native_tls::Error> for EmailError {
    fn from(error: native_tls::Error) -> EmailError {
        EmailError {
            err: error.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct CaptchaError {
    err_message: String,
}

impl reject::Reject for CaptchaError {}

impl CaptchaError {
    pub fn get_err(&self) -> String {
        self.err_message.clone()
    }

    pub fn new<D: std::fmt::Display>(err: D) -> Self {
        CaptchaError {
            err_message: err.to_string(),
        }
    }
}

impl ToString for CaptchaError {
    fn to_string(&self) -> String {
        return self.err_message.clone();
    }
}

impl From<warp::http::Error> for CaptchaError {
    fn from(error: warp::http::Error) -> CaptchaError {
        CaptchaError {
            err_message: error.to_string(),
        }
    }
}

impl From<warp::hyper::Error> for CaptchaError {
    fn from(error: warp::hyper::Error) -> CaptchaError {
        CaptchaError {
            err_message: error.to_string(),
        }
    }
}

// EmailError
#[derive(Debug)]
pub struct ExpiredAuthentication {
    err: String,
}

impl ExpiredAuthentication {
    pub fn new(err: String) -> Self {
        Self { err }
    }
}

impl reject::Reject for ExpiredAuthentication {}

impl ToString for ExpiredAuthentication {
    fn to_string(&self) -> String {
        return self.err.clone();
    }
}
