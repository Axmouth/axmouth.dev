use crate::{
    app::AppState,
    auth_tokens::{decode_token, Claims},
    errors::ExpiredAuthentication,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};
use warp::{reject, Filter};

#[derive(Debug, Clone)]
pub struct MissingAuthentication {}

impl reject::Reject for MissingAuthentication {}
impl MissingAuthentication {
    pub fn get_err(&self) -> String {
        return "Missing authentication data".to_string();
    }
}

#[derive(Debug, Clone)]
pub struct InvalidJWT {
    err: String,
}

impl reject::Reject for InvalidJWT {}
impl InvalidJWT {
    pub fn get_err(&self) -> String {
        return self.err.clone();
    }
}

#[derive(Debug, Clone)]
pub struct RequestValidationFailure {
    err: String,
    validation_errors: ValidationErrors,
}

impl reject::Reject for RequestValidationFailure {}
impl RequestValidationFailure {
    pub fn get_err(&self) -> String {
        let error_msg = validation_errors_to_msg(self.validation_errors.clone());
        return error_msg;
    }
}
trait TitleCase {
    fn title(&self) -> String;
    fn untitle(&self) -> String;
}

impl TitleCase for &str {
    fn title(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }

    fn untitle(&self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
        }
    }
}

fn camel_to_snake(snake: String) -> String {
    snake
        .split('_')
        .map(|s| s.title())
        .collect::<Vec<String>>()
        .join("")
        .as_str()
        .untitle()
        .to_string()
}

fn validation_errors_to_msg(errors: ValidationErrors) -> String {
    let mut error_msg = String::from("");
    for (field, error_kinds) in errors.clone().into_errors() {
        let mut msg_init = format!("Error on field `{}`", camel_to_snake(field.to_string()));
        // error_msg.push_str(msg.as_str());
        match error_kinds {
            validator::ValidationErrorsKind::Struct(errors) => {
                let inner_msg = validation_errors_to_msg(*errors);
                error_msg.push_str(inner_msg.as_str());
            }
            validator::ValidationErrorsKind::List(errors_map) => {
                for (i, errors) in errors_map {
                    msg_init.push_str(format!(" list element {}", i).as_str());
                    let msg_init = validation_errors_to_msg(*errors);
                    error_msg.push_str(msg_init.as_str());
                }
            }
            validator::ValidationErrorsKind::Field(errors) => {
                for error in errors {
                    let inner_msg = match error.code.to_string().as_str() {
                        "length" => {
                            format!(
                                "{}, not within allowed range of {} to {}",
                                msg_init,
                                error.params.get("min").unwrap_or(
                                    &serde_json::value::Value::String(String::from("null"))
                                ),
                                error.params.get("max").unwrap_or(
                                    &serde_json::value::Value::String(String::from("null"))
                                )
                            )
                        }
                        "email" => format!("{}, not a valid email", msg_init),
                        "url" => format!("{}, not a valid url", msg_init),
                        "phone" => format!("{}, not a valid phone number", msg_init),
                        "credit_card" => format!("{}, not a valid credit card number", msg_init),
                        "required" => format!("{}, field is required", msg_init),
                        "required_nested" => format!("{}, field is required", msg_init),
                        "non_control_character" => {
                            format!("{}, requires a non control character", msg_init)
                        }
                        _ => error.message.map_or(format!("{}\n", msg_init), |v| {
                            format!("{}, {}\n", msg_init, v.to_string())
                        }),
                    };
                    error_msg.push_str(inner_msg.as_str());
                    error_msg.push_str("\n");
                }
            }
        }
    }
    error_msg.push_str("\n");
    error_msg
}

#[derive(Debug, Clone)]
struct InsufficientPriviledge {
    err: String,
}

impl reject::Reject for InsufficientPriviledge {}

pub fn with_state(
    state: AppState,
) -> impl Filter<Extract = (AppState,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn auth_filter(
    jwt_secret: String,
) -> impl Filter<Extract = (Claims,), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("authorization")
        .and(warp::any().map(move || jwt_secret.clone()))
        .and_then(|token_opt: Option<String>, secret: String| async move {
            match token_opt {
                Some(token) => match decode_token(&secret, &token) {
                    Ok(claims) => {
                        if claims.is_expired() {
                            Err(reject::custom(ExpiredAuthentication::new(
                                "This JWT token is expired".to_string(),
                            )))
                        } else {
                            Ok(claims)
                        }
                    }
                    Err(err) => Err(reject::custom(InvalidJWT {
                        err: err.to_string(),
                    })),
                },
                None => Err(reject::custom(MissingAuthentication {})),
            }
        })
}

pub fn auth_admin_filter(
    jwt_secret: String,
) -> impl Filter<Extract = (Claims,), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("authorization")
        .and(warp::any().map(move || jwt_secret.clone()))
        .and_then(|token_opt: Option<String>, secret: String| async move {
            match token_opt {
                Some(token) => match decode_token(&secret, &token) {
                    Ok(claims) => {
                        if claims.is_expired() {
                            Err(reject::custom(ExpiredAuthentication::new(
                                "This JWT token is expired".to_string(),
                            )))
                        } else if claims.is_admin() == false || claims.is_for_admin_site() == false
                        {
                            Err(reject::custom(InsufficientPriviledge {
                                err: "You are not authorized to do this".to_string(),
                            }))
                        } else {
                            Ok(claims)
                        }
                    }
                    Err(err) => Err(reject::custom(InvalidJWT {
                        err: err.to_string(),
                    })),
                },
                None => Err(reject::custom(MissingAuthentication {})),
            }
        })
}

pub fn auth_opt_filter(
    jwt_secret: String,
) -> impl Filter<Extract = (Option<Claims>,), Error = warp::Rejection> + Clone {
    warp::header::optional::<String>("authorization")
        .and(warp::any().map(move || jwt_secret.clone()))
        .and_then(|token: Option<String>, secret: String| async move {
            let token = match token {
                None => return Ok(None),
                Some(value) => value,
            };
            match decode_token(&secret, &token) {
                Ok(claims) => {
                    if claims.is_expired() {
                        Ok(None)
                    } else {
                        Ok(Some(claims))
                    }
                }
                Err(err) => Err(reject::custom(InvalidJWT {
                    err: err.to_string(),
                })),
            }
        })
}

pub fn validated_json<T: Validate + DeserializeOwned + Send>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::json().and_then(|json_object: T| async move {
        json_object.validate().map(|_| json_object).map_err(|err| {
            reject::custom(RequestValidationFailure {
                err: err.to_string(),
                validation_errors: err,
            })
        })
    })
}
