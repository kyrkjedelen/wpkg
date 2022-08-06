use std::{ process };

use std::path::{ Path };
use std::process::{ Command };
use std::fmt::{ Display };
use vpkg::{ Config };


fn main() {
    check_program_availiability("dpkg");
    check_program_availiability("apt");


    

    println!("Configurating arguments...");
    let config = match Config::new() {
        Ok(config) => config,
        Err(error) => {
            fatal_error("Configuration error", error);
            return;
        }
    };

    println!("Downloading file...");
    if let Err(error) = vpkg::run(config) {
        fatal_error("Application error", error);
    }

    println!("Download ready!");
}


fn check_program_availiability(prog_name: &str) {
    let output = Command::new(prog_name)
        .arg("--version")
        .output();

    if let Err(_) = output {
        let error_message = format!("Did not find {} installed on your system.", prog_name);
        fatal_error("Configuration error", error_message);
    }
}

fn fatal_error<T: Display>(error_title: &str, error: T) {
    print_error(error_title, error);
    process::exit(1);
}
fn print_error<T: Display>(error_title: &str, error: T) {
    eprintln!("{}: {}", error_title, error);
}