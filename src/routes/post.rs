use actix_web::{Result, Responder, Error, HttpResponse, post};
use actix_multipart::{
    form::{
        tempfile::TempFile,
        MultipartForm
    }
};
use uuid::Uuid;
use std::{env, path::PathBuf};

use crate::database::add_ferret_to_db;

#[derive(Debug, MultipartForm)]
pub struct FerretForm {
    #[multipart(rename = "file")]
    file: TempFile
}

#[post("/submit")]
pub async fn submit_ferret(MultipartForm(form): MultipartForm<FerretForm>) -> Result<impl Responder, Error> {
    let files_dir = env::var("PUBLIC_ASSETS").unwrap();
    let mut path = PathBuf::from(files_dir);

    let uuid = Uuid::new_v4();
    path.push(format!("{}.{}", uuid, form.file.file_name.unwrap().split(".").collect::<Vec<_>>()[1]));

    add_ferret_to_db(uuid, path.to_str().unwrap().to_string()).unwrap();

    form.file.file.persist(&path).unwrap();

    println!("{:?}", &path);

    Ok(HttpResponse::Ok())
}