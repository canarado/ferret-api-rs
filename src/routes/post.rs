use actix_web::{Result, Responder, Error, HttpResponse, post};
use actix_multipart::{
    form::{
        tempfile::TempFile,
        MultipartForm, text::Text
    }
};
use uuid::Uuid;
use std::{env, path::PathBuf};

use crate::util::valid_url_key;

#[derive(Debug, MultipartForm)]
pub struct FerretForm {
    #[multipart(rename = "file")]
    file: TempFile,
    token: Text<String>
}

#[post("/submit")]
pub async fn submit_ferret(MultipartForm(form): MultipartForm<FerretForm>) -> Result<impl Responder, Error> {

    if !valid_url_key(form.token.to_string()) {
        return Ok(HttpResponse::Forbidden().finish())
    }

    let files_dir = env::var("PUBLIC_ASSETS").unwrap();
    let mut path = PathBuf::from(files_dir);

    if let Some(file_name) = form.file.file_name {
        let uuid = Uuid::new_v4();
        path.push(format!("{}.{}", uuid, file_name.split(".").collect::<Vec<_>>()[1]));
    
        form.file.file.persist(&path).unwrap();
    
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::BadRequest().finish())
    }
}