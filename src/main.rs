use std::{collections::HashMap, env};
use actix_web::{HttpServer, App, HttpResponse, HttpRequest, web};

struct AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    println!("Serveur en écoute sur le port {}", port);

    HttpServer::new(|| {
        App::new().app_data(web::Data::new(AppState)).route("/ping", web::get().to(handle_client))
    }).bind(format!("0.0.0.0:{}", port))?.run().await
}

async fn handle_client(_state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let status_code = 200;
    let client_address = req.peer_addr().map(|addr| addr.to_string()).unwrap_or_else(|| "Unknown".to_string());
    let request_headers: HashMap<String, String> = req.headers().iter().map(|(name, value)| (name.as_str().to_string(), value.to_str().unwrap_or("").to_string())).collect();

    let response = format!(
        "HTTP/1.1 {}\r\nContent-Type: application/json\r\n\r\n{{\n\"Request URL\": \"{}\",\n\"Request Method\": \"GET\",\n\"Status Code\": {},\n\"Remote Address\": \"{}\",\n\"Headers\": {:?}\n}}\n",
        status_code, "/ping", status_code, client_address, request_headers
    );

    println!("{}", response);

    HttpResponse::Ok().content_type("application/json").body(response)
}