use std::fs::File;
use std::error::Error;

use std::io::prelude::*;

use super::{
    error,
    CONFIG_PATH,
};

pub struct Package {
    name: String,
    url: String,
    is_active: bool
}

impl Package {
    fn as_string(&self) -> String {
        format!("{} {} {}", self.name, self.url, self.is_active)
    }
    fn new(name: String, url: String, is_active: bool) -> Self {
        Package { name, url, is_active }
    }
}

pub fn add_package_to_config_file(new_package: Package) -> Result<(), Box<dyn Error>> {
    use error::configerror::{Error, ErrorKind};

    let mut config_file = File::open(CONFIG_PATH)?;
    let mut contents = String::new();
    
    config_file.read_to_string(&mut contents)?;

    let mut packages: Vec<Package> = Vec::new();

    packages.push(new_package);

    let mut line_index = 0;
    for line in contents.lines() {
        line_index += 1;
        match make_package_from_string(line) {
            Some(package)  => {
                packages.push(package)
            }
            None => {
                let error_message = format!("package in line {line_index} in the config file is unvalid. Please remove it.");
                return Err(
                    Box::new(
                        Error::new(ErrorKind::BadPackage, error_message)
                    )
                );
            }
        }
    }
    packages.sort_unstable_by(|a,b| {a.name.cmp(&b.name)});

    let mut packages_string = String::new();
    for package in packages {
        packages_string.push_str(&package.as_string())
    }

    config_file.write_all(packages_string.as_bytes())?;

    Ok(())
}

fn make_package_from_string(string: &str) -> Option<Package> {
    let mut slices = string.split(' ');
    let name = slices.next()?.to_string();
    let url = slices.next()?.to_string();
    let is_active = slices.next()?;
    let is_active = match is_active {
        "active" => true,
        "deactive" => false,
        _ => return None,
    };

    if slices.next().is_some() {
        return None;
    }

    Some(
        Package::new(name, url, is_active)
    )
}

#[cfg(test)]
mod tests {
    use super::{ * };

    #[test]
    fn test_parsing_success() -> () {
        let test_package = make_package_from_string("test http://localhost:6969 active").unwrap();
        assert_eq!(test_package.name, "test");
        assert_eq!(test_package.url, "http://localhost:6969");
        assert_eq!(test_package.is_active, true);

        let test_package = make_package_from_string("packagename https:://www.google.com deactive").unwrap();
        assert_eq!(test_package.name, "packagename");
        assert_eq!(test_package.url, "https:://www.google.com");
        assert_eq!(test_package.is_active, false);
    }
    #[test]
    fn test_parsing_few_items() -> () {
        let test_package = make_package_from_string("test active");
        assert!(test_package.is_none());
    }
    #[test]
    fn test_parsing_many_items() -> () {
        let test_package = make_package_from_string("test http://localhost:6969 active hello");
        assert!(test_package.is_none());
    }
    #[test]
    fn test_parsing_unvalid() -> () {
        let test_package = make_package_from_string("test http://localhost:6969 actived");
        assert!(test_package.is_none());
    }
}