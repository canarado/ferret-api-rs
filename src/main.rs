use actix_web::{App, HttpServer, web, middleware};
use actix_files as fs;
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;

mod routes;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    check_env();

    HttpServer::new(|| {

        let cors = Cors::default().allow_any_origin();

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(routes::get::random_ferret)
                    .service(routes::post::submit_ferret)
                    .service(routes::get::get_ferret_by_id)
            )
            .service(fs::Files::new("/cdn", env::var("PUBLIC_ASSETS").unwrap()))
    })
    .bind(("127.0.0.1", env::var("API_PORT").unwrap().parse::<u16>().unwrap()))?
    .run()
    .await
}

fn check_env() {
    env::var("PUBLIC_ASSETS").expect(format_env_error("PUBLIC_ASSETS").as_str());
    env::var("API_PORT").expect(format_env_error("API_PORT").as_str());
    env::var("HOSTNAME").expect(format_env_error("HOSTNAME").as_str());
    env::var("API_KEYS").expect(format_env_error("API_KEYS").as_str());
}

fn format_env_error(variable_name: &'static str) -> String {
    format!("Please set the {} variable in your `.env` file", variable_name)
}