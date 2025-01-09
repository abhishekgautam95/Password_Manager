use serde::Serialize;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use crate::models::PasswordEntry;

const FILE_PATH: &str = "passwords.json";

pub fn save_passwords(passwords: &[PasswordEntry]) {
    let json = serde_json::to_string(passwords).expect("Failed to serialize passwords");
    let mut file = File::create(FILE_PATH).expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");
}

pub fn load_passwords() -> Vec<PasswordEntry> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(FILE_PATH).expect("Failed to read file");
    serde_json::from_str(&data).expect("Failed to deserialize passwords")
}