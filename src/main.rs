use std::fs;
use std::io::Error as stdError;
use std::net::SocketAddr;

use clap::{command, crate_version, Arg};

use http_body_util::{combinators::BoxBody, Full};
use http_body_util::{BodyExt, StreamBody};
use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Method;
use hyper::{body::Bytes, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tokio_util::io::ReaderStream;

use futures_util::TryStreamExt;

static NOTFOUND: &[u8] = b"Not Found";
// static INDEX: &str = "/Users/austinperrine/Desktop/rust/static_server/file_src/index.html";
static INDEX: &str = "./file_src/index.html";

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
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
        .get_matches();

    println!("ip: {:?}", matches.get_one::<String>("ip"));
    println!("port: {:?}", matches.get_one::<String>("port"));
    let ip = matches.get_one::<String>("ip").unwrap();
    let port = matches.get_one::<String>("port").unwrap();

    let addr: SocketAddr = format!("{}:{}", ip, port).parse().unwrap();
    println!("{:?}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_requests))
                .await
            {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn handle_requests(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, stdError>>, stdError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(send_file(INDEX).await),
        _ => Ok(not_found()),
    }
}

async fn send_file(file_name: &str) -> Response<BoxBody<Bytes, stdError>> {
    let file = tokio::fs::File::open(file_name).await;
    if file.is_err() {
        return not_found();
    }

    let mut file: tokio::fs::File = file.unwrap();

    let reader_stream = ReaderStream::new(file);
    let stream_body = StreamBody::new(reader_stream.map_ok(Frame::data));

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(stream_body.boxed())
        .unwrap()
}

fn not_found() -> Response<BoxBody<Bytes, stdError>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(
            Full::new("Not Found".into())
                .map_err(|e| match e {})
                .boxed(),
        )
        .unwrap()
}
