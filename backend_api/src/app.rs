use std::env;
use std::net::SocketAddr;

use warp::{
    self,
    hyper::{header, Method},
    Filter,
};

use crate::cookies::CookieBuilder;
use crate::db::Repo;
use crate::{emails::EmailSender, routes};

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

#[derive(Clone)]
pub struct AppState {
    pub repository: Repo,
    pub jwt_secret: String,
    pub jwt_duration: i64,
    pub captcha_secret: String,
    pub refresh_cookie_builder: CookieBuilder,
    pub static_file_dir: String,
    pub static_file_address: String,
    pub email_sender: EmailSender,
}

pub async fn start() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let static_file_dir = env::var("STATIC_FILE_DIR").expect("STATIC_FILE_DIR must be set");
    let static_file_address =
        env::var("STATIC_FILE_ADDRESS").expect("STATIC_FILE_ADDRESS must be set");
    let repository = Repo::new(database_url).await;

    let bind_address: SocketAddr = env::var("BIND_ADDRESS")
        .expect("BIND_ADDRESS is not set")
        .parse()
        .expect("BIND_ADDRESS is invalid");

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_duration = env::var("JWT_DURATION")
        .expect("JWT_DURATION must be set")
        .parse()
        .unwrap();
    let captcha_secret = env::var("CAPTCHA_SECRET").expect("CAPTCHA_SECRET must be set");
    let refresh_cookie_builder = CookieBuilder::new()
        .with_name("refresh_token".into())
        .with_http_only();

    let email_sender: EmailSender = EmailSender::new();

    let app_state = AppState {
        repository,
        jwt_secret,
        jwt_duration,
        captcha_secret,
        refresh_cookie_builder,
        static_file_dir,
        static_file_address,
        email_sender,
    };

    let cors = warp::cors()
        .allow_any_origin()
        .allow_credentials(true)
        .allow_headers(&[
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ORIGIN,
            header::ACCEPT_LANGUAGE,
            header::ACCEPT_RANGES,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            header::ACCESS_CONTROL_REQUEST_METHOD,
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            header::ACCESS_CONTROL_REQUEST_HEADERS,
            header::ACCESS_CONTROL_REQUEST_METHOD,
        ])
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::PATCH,
            Method::CONNECT,
        ]);

    let routes = routes::routes(app_state)
        .with(cors)
        .with(warp::log(APPLICATION_NAME));

    println!("You can access the server at {}", bind_address);

    warp::serve(routes).run(bind_address).await;
}
