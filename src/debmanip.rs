use std::fs::File;
use std::io;
use std::path::Path;
use std::process::Command;
use std::ffi::OsStr;
use std::error::Error as StdError;

use super::{
    error,
    FOLDER_PATH,
};
use tempfile::Builder;


fn find_deb_name(path: &Path) -> Result<String, Box<dyn StdError>> {
    use error::deberror::{Error, ErrorKind};

    match path.extension() {
        Some(extention) => {
            if !(extention == OsStr::new("deb")) {
                return Err(
                    Box::new(
                        Error::new(
                            ErrorKind::NotDeb, 
                            "file has wrong extention, needs to be \".deb\""
                        )
                    )
                );
            }
        }
        None => {
            return Err(
                Box::new(
                    Error::new(
                        ErrorKind::NotDeb,
                        "file doesn't have extention, needs to be \".deb\""
                    )
                )
            );
        }
    };


    let info = Command::new("dpkg")
        .arg("--info")
        .arg(path)
        .output()?;

    let info = String::from_utf8(info.stdout)?;

    let mut name = "";
    let lines = info.lines();
    for line in lines {
        let line = line.trim();
        if line.starts_with("Package: ") {
            name = &line[9..line.len()-1];
        }
    }

    let name = String::from(name);
    if name.len() == 0 {
        return Err(
            Box::new(
                Error::new(
                    ErrorKind::NoName, 
                    format!("can't find \"package\" row in command \"dpkg --info\" {}", path.display())
                )
            )
        );
    } else {
        return Ok(name);
    }
}

fn download_deb(url: &str) -> Result<(), Box<dyn StdError>> {
    let mut response = reqwest::blocking::get(url)?;

    let mut tempfile = Builder::new()
        .prefix("temp")
        .suffix(".deb")
        .tempfile()?;

    io::copy(&mut response, &mut tempfile)?;

    let file_name = find_deb_name(tempfile.path())?;

    tempfile.close()?;

    let file_path = Path::new(FOLDER_PATH).join(
        Path::new(&file_name).with_extension("deb")
    );


    let mut output = File::create(file_path)?;
    io::copy(&mut response, &mut output)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::{ * };

    #[test]
    fn test_find_deb_name() {
        let name = find_deb_name(Path::new("Minecraft.deb")).unwrap();

        assert_eq!(name, "minecraft-launcher");
    }
}