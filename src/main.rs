mod config;
mod worker;

use actix_web::{
    web::{self, Data},
    App, HttpServer,
};
use anyhow::Context;
use config::Config;
use dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config: Data<Config> =
        Data::new(Config::new().context("Failed to load configuration file!")?);

    let _server = HttpServer::new(move || {
        App::new()
            .app_data(config.clone())
            .service(worker::query)
            .route("/hello", web::get().to(worker::manual_hello))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await;

    Ok(())
}
