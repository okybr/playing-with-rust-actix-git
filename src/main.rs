use std::io::Result;
use std::{fs};
use std::vec::Vec;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/repo/list")]
async fn repo_list() -> impl Responder {
    let mut repositories = Vec::new();

    if let Ok(entries) = fs::read_dir("./repositories") {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Ok(file_name) = file_name.into_string() {
                    repositories.push(file_name);
                }
            }
        }
    }

    HttpResponse::Ok().body(repositories.join("\n"))
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| { App::new().service(repo_list) })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
