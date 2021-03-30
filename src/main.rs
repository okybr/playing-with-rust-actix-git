use std::io::Result;
use std::{fs};
use std::vec::Vec;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use git2::Repository;
use serde_derive::Deserialize;


#[derive(Deserialize)]
struct RepoNewReq {
    name: String,
}


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


#[post("/repo/new")]
async fn repo_new(req: Json<RepoNewReq>) -> impl Responder {
    let repo = Repository::init(
        std::format!("./repositories/{}", req.name));

    match repo {
        Ok(_) => HttpResponse::Ok().body("okay"),
        Err(_) => HttpResponse::Ok().body("error")
    }
}


#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| { App::new().service(repo_list).service(repo_new) })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
