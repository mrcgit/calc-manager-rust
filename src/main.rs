use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
mod utils;
use utils::{CalcManagerPrediction, BcsBonus, compute};


#[derive(Deserialize)]
struct RequestBody {
    predictions: Vec<CalcManagerPrediction>,
    bcsBonus: BcsBonus,
}

#[derive(Serialize)]
struct ResponseBody {
    code: i32,
    result: f64,
    message: String,
}

async fn totalodd(body: web::Json<RequestBody>) -> impl Responder {
    let predictions = &body.predictions;
    let bcsBonus = body.bcsBonus;

    let result = compute(&predictions, &bcsBonus);

    web::Json(ResponseBody {
        code: 0,
        result,
        message: "success".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web service: /totalodd");
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .route("/totalodd", web::post().to(totalodd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
