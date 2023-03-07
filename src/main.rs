use actix_web::{get, App, HttpResponse, HttpServer, Responder};
//import the random fruit function from the lib.rs file
use webdocker::random_movie;

//create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("This is a random best university of the US generator!")
}

//create a function that returns a random best movie
#[get("/university")]
async fn movie() -> impl Responder {
    //print the random movie
    println!("Random University: {}", random_movie());
    HttpResponse::Ok().body(random_movie())
}

//create a function that returns the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    //print the version of the service
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| App::new().service(hello).service(movie).service(version))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
