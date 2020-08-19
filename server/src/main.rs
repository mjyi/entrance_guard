use dotenv::dotenv;
use std::env;
use std::io;

use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,
    Result,Responder
};

#[derive(Clone, Debug)]
struct AppState {
    user_name: String,
    password: String,
    basic_auth: String,
    login_auth: String,
}

async fn index(
    req: HttpRequest,
) -> HttpResponse {
    println!("{:?}", req);
    
    HttpResponse::Ok().body("ok")
}


fn api_login() {

}

fn api_access_card() {

}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let addr = env::var("ADDR").expect("DATABASE_URL must be set");
    let user_name = env::var("USER_NAME").expect("USER_NAME must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");
    let basic_auth = env::var("AUTH").unwrap_or_else(|_| "Basic YXBwOmFwcA==".to_string());

    let state = AppState{ user_name, password, basic_auth, login_auth: String::new()};

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
    })
    .bind(addr)?
    .run()
    .await

}
