use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    middleware::Logger,
    App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use reqwest;
use tokio::{
    fs::File,
    io::{self, AsyncWriteExt},
};

static PORT: u16 = 50001;
static IMAGE_NAME: &'static str = "frente_esquerda_latest.jpg";
#[cfg(debug_assertions)]
static INDEX_FILE: &str = include_str!("../static/index.html");
#[cfg(not(debug_assertions))]
static INDEX_FILE: &str = include_str!("../static/index.html");
#[get("/")]
async fn index(_req: HttpRequest) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::html())
        .body(INDEX_FILE))
}

#[get("/frente_esquerda_snap")]
async fn frente_esquerda_snap() -> impl Responder {
    let url = "http://10.0.66.1:5000/api/frente_esquerda/latest.jpg";
    let response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();

    let mut file = File::create(format!("static/{IMAGE_NAME}")).await.unwrap();
    file.write_all(&bytes).await.unwrap();

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().send_wildcard())
            .wrap(Logger::default())
            .service(index)
            .service(frente_esquerda_snap)
            .service(Files::new("/static", "static").show_files_listing())
    })
    .bind(("127.0.0.1", PORT))?
    .workers(2)
    .run()
    .await
}
