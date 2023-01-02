use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn pong() -> impl Responder {
   HttpResponse::Ok().body("pong")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
       App::new().service(
           web::scope("/")
               .route("ping", web::get().to(pong))
               .route("", web::get().to(|| HttpResponse::NotFound())),
       )
   })
   .workers(6)
   .bind("127.0.0.1:4000")
   .unwrap()
   .run()
   .await
}
