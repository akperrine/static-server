use std::fs;

use clap::{command, crate_version, Arg};
use settings::Settings;

pub mod server;
pub mod settings;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = Settings::set_args().get_matches();
    // let matches = command!()
    //     .version(crate_version!())
    //     .arg(
    //         Arg::new("root")
    //             .index(1)
    //             .value_parser(|s: &str| match fs::metadata(s) {
    //                 Ok(metadata) => {
    //                     if metadata.is_dir() {
    //                         Ok(())
    //                     } else {
    //                         Err("Not directory".to_owned())
    //                     }
    //                 }
    //                 Err(e) => Err(e.to_string()),
    //             })
    //             .help("Root directory"),
    //     )
    //     .arg(
    //         Arg::new("ip")
    //             .long("ip")
    //             .default_value("127.0.0.1")
    //             .help("IP address to bind"),
    //     )
    //     .arg(
    //         Arg::new("port")
    //             .short('p')
    //             .long("port")
    //             .default_value("8000")
    //             .help("Port number"),
    //     );

    println!("ip: {:?}", matches.get_one::<String>("ip"));
    println!("port: {:?}", matches.get_one::<String>("port"));
    let ip = matches.get_one::<String>("ip").unwrap();
    let port = matches.get_one::<String>("port").unwrap();

    let _ = server::start_server(&ip, &port).await;
    Ok(())
}
