use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{post, web, App, HttpResponse, HttpServer, Result};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct Args {
    ints: Vec<i32>,
}

#[derive(Serialize)]
struct MaxResponse {
    max: i32,
}

#[post("/find-max")]
async fn find_max(args: Json<Args>) -> Result<Json<MaxResponse>> {
    let max: &i32 = args.ints.iter().max().unwrap(); // compute max from `ints`
    return Ok(Json(MaxResponse { max: *max })); // return `Max` as json
}

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body("Error: 404, wrong route"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(find_max)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
