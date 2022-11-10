mod list;
mod wol;

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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/wake/{id}")]
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

#[post("/pc")]
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

#[delete("/pc/{id}")]
async fn pc_delete(id: web::Path<String>) -> impl Responder {
    list::delete_item(id.to_string());

    HttpResponse::Ok().body("ok")
}

#[get("/pc")]
async fn pc_list() -> impl Responder {
    let items = list::get_items()
        .into_iter()
        .map(|i| i.name)
        .collect::<Vec<String>>()
        .join(" ");
    format!("{items}")
}
