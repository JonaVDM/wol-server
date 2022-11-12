mod list;
mod wol;

use actix_files as fs;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(wake)
            .service(pc_list)
            .service(pc_new)
            .service(pc_delete)
            .service(
                fs::Files::new("/", "./frontend/dist")
                    .show_files_listing()
                    .use_last_modified(true)
                    .index_file("index.html"),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/api/wake/{id}")]
async fn wake(id: web::Path<String>) -> impl Responder {
    let item = match list::get_item(id.to_string()) {
        Ok(i) => i,
        Err(_) => return HttpResponse::BadRequest().body("Unkown ID"),
    };

    match wol::wol(&item.mac, "255.255.255.255") {
        Ok(_) => HttpResponse::Ok().body("The PC has awoken"),
        Err(_) => HttpResponse::InternalServerError().body("The PC could not be woken"),
    }
}

#[post("/api/pc")]
async fn pc_new(body: String) -> impl Responder {
    let res: Vec<_> = body.split(";").collect();

    if res.len() < 2 {
        return HttpResponse::BadRequest().body("Body needs at least 2 arguemnts");
    }

    match list::add_item(res[0].to_owned(), res[1].to_owned()) {
        Ok(()) => HttpResponse::Ok().body("ok"),
        Err(()) => HttpResponse::InternalServerError().body("Something went wrong"),
    }
}

#[delete("/api/pc/{id}")]
async fn pc_delete(id: web::Path<String>) -> impl Responder {
    list::delete_item(id.to_string());

    HttpResponse::Ok().body("ok")
}

#[get("/api/pc")]
async fn pc_list() -> impl Responder {
    let items = list::get_items()
        .into_iter()
        .map(|i| format!("{};{};{}", i.id, i.name, i.mac))
        .collect::<Vec<String>>()
        .join("\n");
    format!("{items}")
}
