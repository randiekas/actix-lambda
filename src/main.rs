use lambda_web::actix_web::{self, get, App, HttpServer, Responder, web, Result};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};

#[get("/")]
async fn hello() -> Result<impl Responder> {

    Ok(web::Json(serde_json::json!({
        "status": true,
        "message": "hello world"
    })))
    
}

#[get("/hello2")]
async fn hello2() -> Result<impl Responder> {

    Ok(web::Json(serde_json::json!({
        "status": true,
        "message": "hello world 2"
    })))
    
}

#[actix_web::main]
async fn main() -> Result<(),LambdaError> {
    let factory = move || {
        App::new()
            .service(hello)
            .service(hello2)
    };

    if is_running_on_lambda() {
        // Run on AWS Lambda
        run_actix_on_lambda(factory).await?;
    } else {
        // Local server
        HttpServer::new(factory)
            .bind("0.0.0.0:8000")?
            .run()
            .await?;
    }
    Ok(())
}