use rand::seq::{IteratorRandom};
use std::{env, fs::{self, DirEntry, ReadDir}};

pub fn get_files() -> ReadDir {

    fs::read_dir(env::var("PUBLIC_ASSETS").unwrap()).unwrap()

}

pub fn get_file_by_id(id: String) -> Option<DirEntry> {
    let files = get_files();

    for file in files {

        let entry = file.unwrap();

        let file_name = entry.file_name();

        let file_name_string = file_name.to_string_lossy();

        let name = &file_name_string.split(".").collect::<Vec<_>>()[0];
        if name == &&id {
            return Some(entry)
        }
    }

    None
}

pub fn random_file() -> DirEntry {
    let files = get_files();

    files.choose(&mut rand::thread_rng()).unwrap().unwrap()
}

pub fn valid_url_key(key: String) -> bool {
    let keys_var = env::var("API_KEYS").unwrap();
    let known_keys = keys_var.split(",").collect::<Vec<_>>();

    known_keys.contains(&&key.as_str())
}