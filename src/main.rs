use actix_web::{web, App, Responder, HttpServer};
use deadpool_redis::{cmd, Config, Pool};

async fn get_from_redis(key: String, redis_pool: web::Data<Pool>) -> String {
    let mut conn = redis_pool.get().await.unwrap();

    return cmd("GET")
        .arg(&[key])
        .query_async(&mut conn)
        .await.unwrap_or_default();
}

async fn add_to_redis(key: String, val: String, redis_pool: web::Data<Pool>) {
    let mut conn = redis_pool.get().await.unwrap();

    return cmd("SET")
        .arg(&[key, val])
        .execute_async(&mut conn)
        .await.unwrap_or_default();
}

async fn index(path_params: web::Path<(String, u32)>, redis_pool: web::Data<Pool>) -> impl Responder {
    let mut item= get_from_redis(path_params.0.clone(), redis_pool.clone()).await;

    if item.is_empty() {
        add_to_redis(path_params.0.clone(), "42".to_string(), redis_pool.clone()).await;
        item = get_from_redis(path_params.0.clone(), redis_pool.clone()).await;
    }

    format!("Hello {}! id:{} redis_value:{}", path_params.0, path_params.1, item)
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