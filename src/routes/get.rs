use actix_web::{get, Responder, HttpResponse, web, Result};
use serde::Deserialize;
use crate::database::{get_random_ferret};

#[get("/random")]
pub async fn random_ferret() -> Result<impl Responder> {
    let random_ferret = get_random_ferret().unwrap();

    Ok(web::Json(random_ferret))
}