use actix_web::{App, HttpServer, web, middleware};
use actix_files as fs;
use dotenv::dotenv;
use std::env;

mod routes;
mod database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    check_env();

    database::prepare_database().expect("Something went wrong connecting to the Sqlite database");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/api")
                    .service(routes::get::random_ferret)
                    .service(routes::post::submit_ferret)
            )
            .service(fs::Files::new("/cdn", env::var("PUBLIC_ASSETS").unwrap()).show_files_listing())
    })
    .bind(("127.0.0.1", env::var("API_PORT").unwrap().parse::<u16>().unwrap()))?
    .run()
    .await
}

fn check_env() {
    env::var("SQLITE_DB_FILE").expect(format_env_error("SQLITE_DB_FILE").as_str());
    env::var("PUBLIC_ASSETS").expect(format_env_error("PUBLIC_ASSETS").as_str());
    env::var("API_PORT").expect(format_env_error("API_PORT").as_str());
}

fn format_env_error(variable_name: &'static str) -> String {
    format!("Please set the {} variable in your `.env` file", variable_name)
}