use std::io::Error as stdError;
use std::net::SocketAddr;

use futures_util::TryStreamExt;
use http_body_util::StreamBody;
use http_body_util::{combinators::BoxBody, Full};
use hyper::body::Frame;
use hyper::Method;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use tokio::fs::File;
use tokio_util::io::ReaderStream;

use http_body_util::BodyExt;

static INDEX: &str = "./file_src/index.html";

pub async fn start_server(ip: &str, port: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    let file: File = file.unwrap();

    let reader_stream = ReaderStream::new(file);
    let stream_body: StreamBody<futures_util::stream::MapOk<ReaderStream<File>, _>> =
        StreamBody::new(reader_stream.map_ok(Frame::data));

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
