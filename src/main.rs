use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct RequestBody {
    predictions: Vec<f64>,
    bcs_bonus: f64,
}

#[derive(Serialize)]
struct ResponseBody {
    code: i32,
    result: f64,
    message: String,
}

async fn totalodd(body: web::Json<RequestBody>) -> impl Responder {
    let predictions = &body.predictions;
    let bcs_bonus = body.bcs_bonus;

    // Calcola il risultato (sostituisci con la tua logica)
    let result = predictions.iter().sum::<f64>() + bcs_bonus;

    web::Json(ResponseBody {
        code: 0,
        result,
        message: "success".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .route("/totalodd", web::post().to(totalodd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
