use rand::seq::{IteratorRandom};
use std::{env, fs::{self, DirEntry, ReadDir}};

pub fn get_files() -> ReadDir {

    fs::read_dir(env::var("PUBLIC_ASSETS").unwrap()).unwrap()

}

pub fn random_file() -> DirEntry {
    let files = get_files();

    files.choose(&mut rand::thread_rng()).unwrap().unwrap()
}

pub fn check_api_key(key: String) -> bool {
    let keys_var = env::var("API_KEYS").unwrap();
    let known_keys = keys_var.split(",").collect::<Vec<_>>();

    if known_keys.contains(&&key.as_str()) {
        true
    } else {
        false
    }
}