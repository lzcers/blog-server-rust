use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use std::{env, net::SocketAddr};
mod db;
mod services;
use axum_server::tls_rustls::RustlsConfig;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    let blog_db = db::BlogDB::new(&db_url).await.expect("connect blog faild!");
    tracing::debug!("listening on {}", addr);

    let app = Router::new()
        .route("/add_note", post(services::add_note))
        .route("/get_all_notes", get(services::get_all_note))
        .route("/update_note", post(services::update_note))
        .route("/delete_note", post(services::delete_note))
        .layer(Extension(blog_db));

    let config =
        RustlsConfig::from_pem_file(env::var("SSL_CERT").unwrap(), env::var("SSL_KEY").unwrap())
            .await
            .unwrap();

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
