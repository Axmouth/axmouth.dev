pub mod create_super_user;

#[tokio::main]
async fn main() {
    let parsed_args = parse_args();

    match parsed_args {
        CmdType::Help => println!("{}", HELP_TEXT),
        CmdType::CreateSuperUser {
            display_name: Some(display_name),
            email: Some(email),
            password: Some(password),
        } => {
            match create_super_user::create_super_user(display_name.clone(), email, password).await
            {
                Ok(_) => println!("\n\nNew superuser {} has been created!!\n", display_name),
                Err(err) => eprintln!("{}", err.to_string()),
            }
        }
        _ => {}
    }
}

pub enum CommandType {
    CreateSuperUser {
        display_name: Option<String>,
        password: Option<String>,
        email: Option<String>,
    },
    None,
}

#[derive(Debug)]
pub enum CmdType {
    CreateSuperUser {
        display_name: Option<String>,
        email: Option<String>,
        password: Option<String>,
    },
    Help,
    None,
}

pub fn parse_args() -> CmdType {
    let args = std::env::args();
    let mut arg_iter = args.into_iter().peekable();
    let mut cmd: CmdType;
    arg_iter.next();

    match arg_iter.next() {
        Some(value) => match value.as_str() {
            "create-super-user" => {
                cmd = CmdType::CreateSuperUser {
                    password: None,
                    display_name: None,
                    email: None,
                }
            }
            "help" => cmd = CmdType::Help,
            _ => panic!("Unknown argument: {}", value),
        },
        None => panic!("Not enough arguments, try --help"),
    }

    while let Some(next_arg) = arg_iter.next() {
        match cmd {
            CmdType::CreateSuperUser {
                ref mut email,
                ref mut display_name,
                ref mut password,
            } => match next_arg.as_str() {
                "--password" | "-p" => match arg_iter.next() {
                    Some(value) => {
                        *password = Some(value);
                    }
                    None => panic!("Not enough arguments, try --help"),
                },
                "--display-name" | "-d" => match arg_iter.next() {
                    Some(value) => {
                        *display_name = Some(value);
                    }
                    None => panic!("Not enough arguments, try --help"),
                },
                "--email" | "-e" => match arg_iter.next() {
                    Some(value) => {
                        *email = Some(value);
                    }
                    None => panic!("Not enough arguments, try --help"),
                },
                _ => panic!("Unknown argumet: {}", next_arg),
            },
            _ => panic!("Unknown argumet: {}", next_arg),
        }
    }

    match &cmd {
        CmdType::CreateSuperUser {
            password,
            display_name,
            email,
        } => {
            let mut error_found = false;
            if let None = password {
                error_found = true;
                eprintln!("No argument for Password(-p/--password) provided")
            }
            if let None = display_name {
                error_found = true;
                eprintln!("No argument for Display Name(-d/--display-name) provided")
            }
            if let None = email {
                error_found = true;
                eprintln!("No argument for Email(-e/--email) provided")
            }
            if error_found {
                panic!()
            }
        }
        _ => {}
    }

    return cmd;
}

pub static HELP_TEXT: &str = "Commands:
    create-super-user:
        -p/--password <password>
        -d/--display-name <display name>
        -e/--email <email>
    help (this message)
    ";
