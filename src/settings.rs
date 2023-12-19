use std::{error::Error as stdError, fs};

use clap::{command, crate_version, Arg, Command};
pub struct Settings {
    port: String,
    ip: String,
}

impl Settings {
    pub fn set_args() -> Command {
        command!()
            .version(crate_version!())
            .arg(
                Arg::new("root")
                    .index(1)
                    .value_parser(|s: &str| match fs::metadata(s) {
                        Ok(metadata) => {
                            if metadata.is_dir() {
                                Ok(())
                            } else {
                                Err("Not directory".to_owned())
                            }
                        }
                        Err(e) => Err(e.to_string()),
                    })
                    .help("Root directory"),
            )
            .arg(
                Arg::new("ip")
                    .long("ip")
                    .default_value("127.0.0.1")
                    .help("IP address to bind"),
            )
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .default_value("8000")
                    .help("Port number"),
            )
    }
    // pub fn get() -> Result<Settings, Box<dyn stdError>> {
    // Ok(())
    // }
}
