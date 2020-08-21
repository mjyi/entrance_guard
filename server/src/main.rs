#[macro_use] extern crate log;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::{env, io, sync::Mutex};

use actix_web::{http, middleware, web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use actix_files::Files;
use anyhow::{anyhow, Result};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Clone, Debug)]
struct AppState {
    user_name: String,
    password: String,
    basic_auth: String,
    access_auth: Option<String>,
    company_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResp<T> {
    #[serde(rename = "responseCode")]
    response_code: Option<i32>,
    message: String,
    data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Passports {
    #[serde(rename = "qrCode")]
    qr_code: String,
    status: i32,
}

#[derive(Deserialize)]
struct AuthRequest {
    reload: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RtData {
    code: i32,
    msg: String,
    data: Option<String>,
}

impl RtData {
    fn new(code: i32, msg: String, data: Option<String>) -> Self {
        RtData { code, msg, data }
    }
}

impl Display for RtData {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

async fn passports(
    web::Query(info): web::Query<AuthRequest>,
    state: web::Data<Mutex<AppState>>,
) -> HttpResponse {
    let mut state = state.lock().unwrap();
    let mut reload;
    reload = info.reload.unwrap_or(false);
    if !reload {
        reload = state.access_auth.is_none();
    }

    let mut access_auth = state.access_auth.clone().unwrap_or(String::new());
    if reload {
        let result = api_login(&state.user_name, &state.password, &state.basic_auth).await;
        if let Err(e) = result {
            return HttpResponse::Ok().json(RtData::new(3, format!("{:?}", e), None));
        }
        access_auth = result.unwrap();
        state.access_auth = Some(access_auth.clone());
    }

    let mut code = 0;
    let mut qr_code = None;
    let result = api_entrance_guard(&access_auth, &state.company_id).await;
    if let Err(_) = result {
        code = 3;
    } else {
        qr_code = Some(result.unwrap().clone())
    }

    HttpResponse::Ok().json(RtData::new(code, "".into(), qr_code))
}

async fn api_login(user_name: &str, password: &str, auth: &str) -> Result<String> {
    let resp: ApiResp<String> = reqwest::Client::new()
        .get(" http://heda.greentownit.com:8888/app/user_center/token")
        .header(reqwest::header::AUTHORIZATION, auth)
        .query(&[
            ("grantType", "password"),
            ("username", user_name),
            ("password", password),
        ])
        .send()
        .await?
        .json()
        .await?;

    if resp.response_code == Some(0) {
        if let Some(data) = resp.data {
            return Ok(data);
        }
    }
    Err(anyhow!("{}", resp.message))
}

async fn api_entrance_guard(auth: &str, company_id: &str) -> Result<String> {
    let resp: ApiResp<Passports> = reqwest::Client::new()
        .get("http://heda.greentownit.com:8888/app/entrance_guard/2/passports/me")
        .header(reqwest::header::AUTHORIZATION, auth)
        .query(&[("companyId", company_id)])
        .send()
        .await?
        .json()
        .await?;

    if resp.response_code == Some(0) {
        if let Some(data) = resp.data {
            return Ok(data.qr_code);
        }
    }
    Err(anyhow!("{}", resp.message))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    pretty_env_logger::init();
    dotenv().ok();
    let addr = env::var("ADDR").expect("DATABASE_URL must be set");
    let user_name = env::var("USER_NAME").expect("USER_NAME must be set");
    let password = env::var("PASSWORD").expect("PASSWORD must be set");
    let basic_auth = env::var("AUTH").unwrap_or_else(|_| "Basic YXBwOmFwcA==".to_string());
    let company_id = env::var("ID").expect("ID must be set");

    let state = AppState {
        user_name,
        password,
        basic_auth,
        company_id,
        access_auth: None,
    };

    let data = web::Data::new(Mutex::new(state));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::new()
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish())
            // .route("/", web::get().to(index))
            .route("/passports", web::get().to(passports))
            .service(Files::new("/", "./entrance-guard").index_file("index.html"))
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind(addr)?
    .run()
    .await
}
