use actix_web::{web, App, HttpServer, Responder, HttpResponse};
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
    let bcsBonus = &body.bcsBonus;

    // Calculate the result
    match compute(&predictions, &bcsBonus) {
        Ok(result) => web::Json(ResponseBody {
            code: 0,
            result,
            message: "success".to_string(),
        }),
        Err(err) => {
            let error_message = format!("Error: {}", err);
            web::HttpResponse::InternalServerError().json(ResponseBody {
                code: 1,
                result: 0.0,
                message: error_message,
            })
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web service: /totalodd");
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(10000))
            .route("/totalodd", web::post().to(totalodd))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
