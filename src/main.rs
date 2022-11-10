use std::io;

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};

mod wol;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(wake)
            .service(pc_list)
            .service(pc_new)
            .service(pc_delete)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/wake/{id}")]
async fn wake(_id: web::Path<String>) -> impl Responder {
    match wol::wol("18:c0:4d:c0:72:7f", "255.255.255.255") {
        Ok(_) => HttpResponse::Ok().body("The PC has awoken"),
        Err(_) => HttpResponse::InternalServerError().body("The PC could not be woken"),
    }
}

#[post("/pc")]
async fn pc_new(body: String) -> impl Responder {
    format!("Adding pc with mac: {body}")
}

#[delete("/pc/{id}")]
async fn pc_delete(id: web::Path<String>) -> impl Responder {
    format!("Deleting pc with id: {id}")
}

#[get("/pc")]
async fn pc_list() -> impl Responder {
    format!("A list of pc's")
}
