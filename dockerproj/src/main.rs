use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dockerproj::random_name;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the docker project!")
}

//create a function that returns a random name
#[get("/name")]
async fn name() -> impl Responder {
    //print the random fruit
    println!("Name is: {}", random_name());
    HttpResponse::Ok().body(random_name())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(name)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
