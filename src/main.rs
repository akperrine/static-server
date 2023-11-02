use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::{thread, env, fs};

fn main() {

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                thread::spawn(move || {
                    println!("accepted new connection ({})", _stream.peer_addr().unwrap());

                    handle_connection(&_stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

}

fn handle_connection(mut stream: &TcpStream) {
    let buff: &mut [u8; 1024] = &mut [0; 1024];     
    stream.read(buff);


    let http_req = parse_http_request(buff).unwrap();

    if http_req.path == "/" {
        println!("hit main path");
        stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
    } else if http_req.path.starts_with("/echo") {
        let response_body = http_req.path.split("/echo/").nth(1).unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response_body.len(), response_body);
        println!("{}", response);
        
        stream.write_all(response.as_bytes());
    } 
    else if http_req.path.starts_with("/user-agent") {
        let resp = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",
            http_req.user_agent.len(),
            http_req.user_agent
        );
        stream.write_all(resp.as_bytes());
    } else if http_req.path.starts_with("/files") {

        let file_name = http_req.path.split("files/").nth(1).unwrap();
    
        let args: Vec<_> = env::args().collect();

        let mut file_path = String::new();
    
        let mut args_iterator = args.iter();
        
        while let Some(arg) = args_iterator.next() {
            if arg == "--directory" {
                if let Some(next_arg) = args_iterator.next() {
                    file_path = (*next_arg).to_string();
                    println!("{}", file_path);
                }
                let file_name = http_req.path.split("files/").nth(1).unwrap();
                println!("{}", file_name);
                let args: Vec<_> = env::args().collect();
                let mut file_path = String::new();
                let mut args_iterator = args.iter();

                while let Some(arg) = args_iterator.next() {
                    println!("arg: {}", arg);
                    if arg == "--directory" {
                        if let Some(next_arg) = args_iterator.next() {
                            file_path = (*next_arg).to_string();
                            println!("file path: {}", file_path);
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        
        let path_to_file = Path::new(&file_path).join(Path::new(file_name));

        if http_req.method == "GET" {
            if path_to_file.is_file() {
                let content = fs::read_to_string(path_to_file).unwrap();
                println!("content: {}", content);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n",
                    content.len(), &content
                );
                stream.write_all(response.as_bytes());
            } else {
                stream.write_all("HTTP/1.1 404 NOT FOUND\r\n\r\n".to_string().as_bytes());
            } 
        }  else if http_req.method == "POST" {
           let mut file = fs::File::create(path_to_file).unwrap();
           file.write_all(http_req.body.as_bytes()).unwrap();

           let resp = format!(
            "HTTP/1.1 201 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n",
            http_req.body.len(),
            http_req.body 
            );
            
            stream.write_all(resp.as_bytes());
        } 
    }
    else {
        stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes());
    }
}

#[derive(Debug, Default)]
struct HttpRequest {
    method: String,
    path: String,
    version: String,
    host: String,
    user_agent: String,
    body: String
}

fn parse_http_request(request_buff: &[u8]) -> Result<HttpRequest, Box<dyn Error>> {
    let str_buff = String::from_utf8(request_buff.to_vec()).unwrap();

    println!("req {}", str_buff);

    let mut http_req = HttpRequest::default();

    let http_lines: Vec<&str> = str_buff.split("\r\n").collect();
    let http_line = http_lines[0];
    let http_line_split: Vec<&str> = http_line.split_ascii_whitespace().collect();
    // println!("{:?}", http_line_split);

    http_req.method = http_line_split[0].to_string();
    http_req.path = http_line_split[1].to_string();
    http_req.version = http_line_split[2].to_string();

    for line in &http_lines {
        if line.contains(":") {
            let line_split: Vec<&str> = line.splitn(2, ':').collect();
            let header_name = line_split[0].trim();
            let header_value = line_split[1].trim();
            
            if header_name.eq_ignore_ascii_case("Host") {
                http_req.host = header_value.to_string();
            }

            if header_name.eq_ignore_ascii_case("User-Agent") {
                http_req.user_agent = header_value.to_string();
            }
        }
    }

    http_req.body = http_lines[http_lines.len() - 1].trim_end_matches(char::from(0)).to_string();
    println!("host: {}, user-agent: {}", http_req.host, http_req.user_agent);
    println!("baller");
    println!("body: {}", http_req.body);  

    Ok(http_req)
}
