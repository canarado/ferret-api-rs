use actix_web::{get, Responder, web, Result};
use serde::{Serialize};
use std::env;

use crate::util;

#[derive(Serialize)]
struct Ferret {
    url: String
}

#[get("/random")]
pub async fn random_ferret() -> Result<impl Responder> {
    let file = util::random_file();

    Ok(web::Json(Ferret {
        url: format!("{}/cdn/{}", env::var("HOSTNAME").unwrap(), file.file_name().to_str().unwrap())
    }))
}