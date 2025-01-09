use rpassword::read_password;
use std::io;

use crate::{encryption, models::PasswordEntry, storage};

pub fn run_cli() {
    let mut passwords = storage::load_passwords();

    println!("Welcome to the Password Manager!");

    // Ask for the master password
    println!("Enter your master password:");
    let master_password = read_password().expect("Failed to read password");
    let key = encryption::derive_key(&master_password);

    loop {
        println!("\nChoose an option:");
        println!("1. Add a new password");
        println!("2. View saved passwords");
        println!("3. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                println!("Enter a name for the password:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read input");
                let name = name.trim().to_string();

                println!("Enter the password to save:");
                let password = read_password().expect("Failed to read password");

                let (encrypted_password, nonce) = encryption::encrypt_password(&key, &password);
                passwords.push(PasswordEntry {
                    name,
                    encrypted_password,
                    nonce,
                });
                storage::save_passwords(&passwords);
                println!("Password saved successfully!");
            }
            "2" => {
                if passwords.is_empty() {
                    println!("No passwords saved yet.");
                } else {
                    println!("Saved passwords:");
                    for entry in &passwords {
                        let decrypted_password = encryption::decrypt_password(
                            &key,
                            &entry.encrypted_password,
                            &entry.nonce,
                        );
                        println!("{}: {}", entry.name, decrypted_password);
                    }
                }
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}