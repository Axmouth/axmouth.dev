use std::net::SocketAddr;
use std::{env, sync::Arc};

use backend_repo_pg::pg_util::{get_pg_pool, PgRepo};
use headers::HeaderValue;

use crate::{emails::EmailSender as EmailSenderInner, routes};
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

use tera::Tera;

const APPLICATION_NAME: &str = env!("CARGO_PKG_NAME");

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub repo: PgRepo,
    pub jwt_secret: String,
    pub jwt_duration: i64,
    pub captcha_secret: String,
    pub static_file_dir: String,
    pub static_file_address: String,
    pub email_sender: EmailSenderInner,
    pub origin: Vec<HeaderValue>,
}

pub trait JwtSecret {
    fn jwt_secret(&self) -> &str;
}

pub trait JwtDuration {
    fn jwt_duration(&self) -> i64;
}

pub trait CaptchaSecret {
    fn captcha_secret(&self) -> &str;
}

pub trait StaticFileDir {
    fn static_file_dir(&self) -> &str;
}

pub trait StaticFileAddress {
    fn static_file_address(&self) -> &str;
}

pub trait EmailSender {
    fn email_sender(&self) -> &EmailSenderInner;
}

pub struct JwtSecretImpl(pub String);

impl JwtSecret for JwtSecretImpl {
    fn jwt_secret(&self) -> &str {
        &self.0
    }
}

pub struct JwtDurationImpl(pub i64);

impl JwtDuration for JwtDurationImpl {
    fn jwt_duration(&self) -> i64 {
        self.0
    }
}

pub struct CaptchaSecretImpl(pub String);

impl CaptchaSecret for CaptchaSecretImpl {
    fn captcha_secret(&self) -> &str {
        &self.0
    }
}

pub struct StaticFileDirImpl(pub String);

impl StaticFileDir for StaticFileDirImpl {
    fn static_file_dir(&self) -> &str {
        &self.0
    }
}

pub struct StaticFileAddressImpl(pub String);

impl StaticFileAddress for StaticFileAddressImpl {
    fn static_file_address(&self) -> &str {
        &self.0
    }
}

pub struct EmailSenderImpl(pub EmailSenderInner);

pub async fn app_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let static_file_dir = env::var("STATIC_FILE_DIR").expect("STATIC_FILE_DIR must be set");
    let static_file_address =
        env::var("STATIC_FILE_ADDRESS").expect("STATIC_FILE_ADDRESS must be set");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_duration = env::var("JWT_DURATION")
        .expect("JWT_DURATION must be set")
        .parse()
        .expect("Failed to parse JWT_DURATION");
    let captcha_secret = env::var("CAPTCHA_SECRET").expect("CAPTCHA_SECRET must be set");
    let repo = get_pg_pool(database_url, 64).await;
    let origin = env::var("ORIGIN")
        .expect("ORIGIN must be set")
        .split(',')
        .map(|s| s.parse().expect("Failed to parse ORIGIN"))
        .collect();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "backend_api=debug,tower_http=debug")
    }

    let email_sender: EmailSenderInner = EmailSenderInner::new();

    AppState {
        repo,
        jwt_secret,
        jwt_duration,
        captcha_secret,
        static_file_dir,
        static_file_address,
        email_sender,
        origin,
    }
}

impl EmailSender for EmailSenderImpl {
    fn email_sender(&self) -> &EmailSenderInner {
        &self.0
    }
}

pub type DynJwtSecret = Arc<dyn JwtSecret + Send + Sync>;
pub type DynJwtDuration = Arc<dyn JwtDuration + Send + Sync>;
pub type DynCaptchaSecret = Arc<dyn CaptchaSecret + Send + Sync>;
pub type DynStaticFileDir = Arc<dyn StaticFileDir + Send + Sync>;
pub type DynStaticFileAddress = Arc<dyn StaticFileAddress + Send + Sync>;
pub type DynEmailSender = Arc<dyn EmailSender + Send + Sync>;

pub async fn start() {
    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let app_state = app_state().await;

    tracing_subscriber::fmt::init();
    tracing::debug!("listening on {}", bind_address);

    println!("You can access the server at {}", bind_address);

    let app = routes::router(app_state).into_make_service_with_connect_info::<SocketAddr, _>();
    axum::Server::bind(&bind_address)
        .serve(app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start axum server");
}

#[cfg(unix)]
pub async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;

    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;
        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }
    println!("signal received, starting graceful shutdown")
}

#[cfg(windows)]
pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
    println!("signal received, starting graceful shutdown")
}
