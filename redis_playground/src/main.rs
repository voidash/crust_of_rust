pub mod middleware;

use actix_web::{ web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use middleware::ActixCache;
use serde::Deserialize;
use actix_redis::{RedisActor,Command, resp_array, RespValue};
// configuration for managing the routes

//adding the middleware

async fn guarded_hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(Debug,Deserialize)]
pub struct Parameters {
    search : Option<String>
}

async fn summary(query: web::Query<Parameters>) -> impl Responder {
    format!("the query is {:?}", query.into_inner().search)
}

async fn exonerator(path: web::Path<(String, String)>) -> impl Responder {
    let (ip, date) = path.into_inner();
    format!("ok, the ip is {} and date is {}" ,ip, date)
}

async fn summary_with_handler(_req: HttpRequest) -> impl Responder {
    format!("summary with handle {:?}", _req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = RedisActor::start("127.0.0.1:6379");
    let command = resp_array!["SET", "test", "from_rust"];

    let response = addr.send(Command(command)).await.unwrap();
    match response {
        Ok(message) => println!("{:?}", message),
        Err(_) => {}
    }


    HttpServer::new(|| {
        App::new()
            .wrap(ActixCache)
            .service(
                web::scope("")
                    .route("/hey", web::get().to(guarded_hello))
                    .route("/check-health", web::get().to(health_check))
                    .route("/exonerator/{ip}/{date}", web::get().to(exonerator))
                    .route("/summary", web::get().to(summary))
                    .route("/summary-handler", web::get().to(summary_with_handler))
            )

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}