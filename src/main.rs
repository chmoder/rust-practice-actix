use actix_web::{web, App, Responder, HttpServer};
use deadpool_redis::{cmd,Config, Pool};

async fn index(info: web::Path<(String, u32)>, redis_pool: web::Data<Pool>) -> impl Responder {
    let mut conn = redis_pool.get().await.unwrap();

    let item: String = cmd("GET")
        .arg(&["tcross"])
        .query_async(&mut conn)
        .await.unwrap_or_default();

    format!("Hello {}! id:{} hash:{}", info.0, info.1, item)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let cfg = Config::from_env("prefix").unwrap();
    let pool = cfg.create_pool().unwrap();

    HttpServer::new(move || App::new().data(pool.clone()).service(
        web::resource("/{name}/{id}/index.html").to(index))
    )
        .workers(num_cpus::get())
        .bind("0.0.0.0:8080")?
        .run()
        .await
}