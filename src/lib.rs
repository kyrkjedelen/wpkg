use std::{ io, env };

use std::error::Error as StdError;
//use std::path::Path;

mod file; mod error; mod debmanip;


const PROGRAM_NAME: &str = "wpkg";

const DEPENDENCIES: [&str; 2] = [ "dpkg", "apt" ];

const NEEDED_PATHS: [&str; 2] = [ 
    "/env/wpkg/debs/",
    "/env/wpkg/config.json",
];

const FOLDER_PATH: &str = NEEDED_PATHS[0];
const CONFIG_PATH: &str = NEEDED_PATHS[1];

pub fn run(config: Config) -> Result<(), Box<dyn StdError>> {

    match config.action {
        Action::Install(url) => {
            //download_file(&url, "temp.deb")?;
            println!("Download sucessfull.");
        },
        _ => {
            eprintln!("Temperror: not yet added functionality.");
        }
    }

    Ok(())
}

pub enum Action {
    Install(String),
    Update(String),
    Upgrade(String),
    Delete(String),
    Activate(String),
    Deactivate(String),
    List(ListOption),
}

pub enum ListOption {
    Installed,
    Upgradeable,
    Activated,
    Deactivated,
}

pub struct Config {
    pub action: Action,
}

impl Config {
    pub fn new() -> Result<Config, String> {
        let args = get_args();
        if args.len() < 3 {
            return Err("not enough arguments".to_string())
        }

        let second_argument = args[2].to_lowercase();

        let action = match args[1].as_str() {
            "install" => Action::Install(second_argument),
            "update" => Action::Update(second_argument),
            "upgrade" => Action::Upgrade(second_argument),
            "delete" => Action::Delete(second_argument),
            "activate" => Action::Activate(second_argument),
            "deactivate" => Action::Deactivate(second_argument),
            "list" => Action::List(match second_argument.as_str() {
                "installed" => ListOption::Installed,
                "upgradeable" => ListOption::Upgradeable,
                "activated" => ListOption::Activated,
                "deactivated" => ListOption::Deactivated,
                other => return Err(format!("list argument {} not recognized", other)),
            }),
            other => return Err(format!("argument {} not recognized", other)),
        };

        Ok(
            Config {
                action
            }
        )
    }
}

fn get_args() -> Vec<String> {
    env::args().collect()
}





