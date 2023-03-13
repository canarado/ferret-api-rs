use actix_web::{get, Responder, web, Result, HttpResponse};
use serde::{Serialize};
use std::env;

use crate::util::{self, get_file_by_id};

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

#[get("/ferret/{id}")]
pub async fn get_ferret_by_id(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    if let Some(file) = get_file_by_id(id) {
        return HttpResponse::Ok().json(Ferret {
            url: format!("{}/cdn/{}", env::var("HOSTNAME").unwrap(), file.file_name().to_str().unwrap())
        })
    } else {
        return HttpResponse::NotFound().finish()
    }
}