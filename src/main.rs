use std::{collections::HashMap, env, 
    io::{self, BufRead, Write}, 
    net::{TcpListener, TcpStream},};

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Serveur en écoute sur le port {}", port);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle_client(stream);
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut reader = io::BufReader::new(&stream);
    let mut request_line = String::new();

    if reader.read_line(&mut request_line).is_ok() {
        if request_line.starts_with("GET /ping HTTP/1.1") {
            let mut headers = HashMap::new();

            let mut lines = reader.lines();
            while let Some(Ok(line)) = lines.next() {
                if line.trim().is_empty() {
                    break;
                }
                if let Some((name, value)) = parse_header(&line) {
                    headers.insert(name.to_string(), value.to_string());
                }
            }

            let request_url = "/ping".to_string();
            let request_method = "GET".to_string();
            let status_code = 200;
            let client_address = stream.peer_addr().map(|addr| addr.to_string()).unwrap_or("Unknown".to_string());

            let response = format!(
                "HTTP/1.1 {}\r\nContent-Type: application/json\r\n\r\n{{\n\"Request URL\": \"{}\",\n\"Request Method\": \"{}\",\n\"Status Code\": {},\n\"Remote Address\": \"{}\",\n\"Headers\": {:?}\n}}\n",
                status_code, request_url, request_method, status_code, client_address, headers
            );

            println!("{}", response);

            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Erreur lors de l'écriture de la réponse : {}", e);
            }
        } else {
            let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";

            if let Err(e) = stream.write_all(response.as_bytes()) {
                eprintln!("Erreur lors de l'écriture de la réponse : {}", e);
            }
        }
    }
}

fn parse_header(header: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = header.splitn(2, ':').collect();
    if parts.len() == 2 {
        let name = parts[0].trim();
        let value = parts[1].trim();
        Some((name, value))
    } else {
        None
    }
}
