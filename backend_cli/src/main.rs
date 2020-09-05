// mod create_super_user;

fn main() {
    println!("Hello, world!");
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
pub struct CmdArgs {
    pub worker_number: Option<usize>,
    pub input_file_paths: Option<Vec<String>>,
    pub output_file_path: Option<String>,
    pub category_file_path: Option<String>,
    pub request_timeout: Option<u64>,
    pub category_key_column: Option<usize>,
    pub input_category_key_column: Option<usize>,
    pub ua_header: Option<String>,
    pub help: Option<()>,
}

impl CmdArgs {
    pub fn new() -> Self {
        Self {
            worker_number: None,
            input_file_paths: None,
            output_file_path: None,
            category_file_path: None,
            request_timeout: None,
            category_key_column: None,
            input_category_key_column: None,
            ua_header: None,
            help: None,
        }
    }
}

pub fn parse_args() -> CmdArgs {
    let mut args = std::env::args();
    println!("{:?}", args);
    let mut arg_values = CmdArgs::new();
    if args.len() == 2 {
        let arg_text = args.nth(1).unwrap();
        if !arg_text.starts_with("-") {
            arg_values.input_file_paths = Some(vec![arg_text]);
            return arg_values;
        }
    } else if args.len() > 2 {
        // let arg_iter = args.into_iter();
        let mut cmd_exist = false;
        let mut path_list = vec![];
        for arg in args {
            if arg.starts_with("-") {
                cmd_exist = true;
                break;
            }
            path_list.push(arg);
        }
        if !cmd_exist {
            arg_values.input_file_paths = Some(path_list);
            return arg_values;
        }
    }

    let args = std::env::args();
    let mut arg_iter = args.into_iter().peekable();

    arg_iter.next();
    loop {
        let arg = arg_iter.next();
        if arg.is_none() {
            break;
        }
        let arg_text = arg.unwrap();
        if arg_text == "--help" || arg_text == "-h" {
            arg_values.help = Some(());
        } else if arg_text == "-n" {
            let arg = arg_iter.next();
            if arg.is_none() {
                panic!("Expected a number for worker threads after {}", arg_text);
            }
            let arg_text = arg.unwrap();
            let n_parse = arg_text.parse();
            if n_parse.is_err() {
                panic!("Failed to parse {} into number", arg_text);
            }
            let n: usize = n_parse.unwrap();
            arg_values.worker_number = Some(n);
        } else if arg_text == "-o" || arg_text == "--output" {
            let arg = arg_iter.next();
            if arg.is_none() {
                panic!("Expected an output file path after {}", arg_text);
            }
            let arg_text = arg.unwrap();
            arg_values.output_file_path = Some(arg_text);
        } else if arg_text == "-i" || arg_text == "--input" {
            let mut path_list = vec![];
            while arg_iter.peek().is_some() && !arg_iter.peek().unwrap().starts_with("-") {
                let arg = arg_iter.next();
                let arg_text = arg.unwrap();
                path_list.push(arg_text);
            }
            if path_list.len() == 0 {
                panic!("Expected an input file path after {}", arg_text);
            }
            arg_values.input_file_paths = Some(path_list);
        } else if arg_text == "--input-cat-col" {
            let arg = arg_iter.next();
            if arg.is_none() {
                panic!("Expected an index of category column after {}", arg_text);
            }
            let arg_text = arg.unwrap();
            let n_parse = arg_text.parse();
            if n_parse.is_err() {
                panic!("Failed to parse {} into number", arg_text);
            }
            let n: usize = n_parse.unwrap();
            arg_values.input_category_key_column = Some(n);
        } else if arg_text == "--cat-key-col" {
            let arg = arg_iter.next();
            if arg.is_none() {
                panic!(
                    "Expected an index of category key column after {}",
                    arg_text
                );
            }
            let arg_text = arg.unwrap();
            let n_parse = arg_text.parse();
            if n_parse.is_err() {
                panic!("Failed to parse {} into number", arg_text);
            }
            let n: usize = n_parse.unwrap();
            arg_values.category_key_column = Some(n);
        } else if arg_text == "--timeout" || arg_text == "-t" {
            let arg = arg_iter.next();
            if arg.is_none() {
                panic!("Expected a request timeout after {}", arg_text);
            }
            let arg_text = arg.unwrap();
            let n_parse = arg_text.parse();
            if n_parse.is_err() {
                panic!("Failed to parse {} into number", arg_text);
            }
            let n: u64 = n_parse.unwrap();
            arg_values.request_timeout = Some(n);
        } else if arg_text == "-u" || arg_text == "--ua-header" {
            let arg = arg_iter.next();
            if arg.is_none() {
                panic!("Expected an user agent header after {}", arg_text);
            }
            let arg_text = arg.unwrap();
            arg_values.ua_header = Some(arg_text);
        } else {
            // panic!("Unknown argument {} provided", arg_text);
        }
    }

    arg_values
}

pub static HELP_TEXT: &str = "HELP TEXT WIP";
