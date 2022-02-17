use validator::ValidationErrors;

#[derive(Debug, Clone)]
pub struct MissingAuthentication {}

impl MissingAuthentication {
    pub fn get_err(&self) -> String {
        "Missing authentication data".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct InvalidJWT {
    err: String,
}

impl InvalidJWT {
    pub fn new(err: String) -> Self {
        InvalidJWT { err }
    }

    pub fn get_err(&self) -> String {
        self.err.clone()
    }
}

#[derive(Debug, Clone)]
pub struct RequestValidationFailure {
    err: String,
    validation_errors: ValidationErrors,
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
}

pub fn validation_errors_to_msg(errors: ValidationErrors) -> String {
    let mut error_msg = String::from("");
    for (field, error_kinds) in errors.into_errors() {
        let mut msg_init = format!("Error on field `{}`", camel_to_snake(field.to_string()));

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
                            format!("{}, {}\n", msg_init, v)
                        }),
                    };
                    error_msg.push_str(inner_msg.as_str());
                    error_msg.push('\n');
                }
            }
        }
    }
    error_msg.push('\n');
    error_msg
}

#[derive(Debug, Clone)]
struct InsufficientPriviledge {
    err: String,
}
