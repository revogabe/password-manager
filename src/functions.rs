use std::io::Write;
use std::io::Read;
use std::fs::File;
use std::{path::Path, io};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

extern crate clipboard;
use crate::PasswordEntry;

const PASSWORD_FILE: &str = "passwords.json";

pub(crate) fn add_password(username: String, password: String, url: String) -> io::Result<()> {
    let mut passwords: Vec<PasswordEntry> = read_passwords()?;

    let entry: PasswordEntry = PasswordEntry {
        username,
        password,
        url,
    };

    passwords.push(entry);
    save_passwords(&passwords)
}

fn read_passwords() -> io::Result<Vec<PasswordEntry>> {
    if !Path::new(PASSWORD_FILE).exists() {
        return Ok(vec![]);
    }

    let mut file: File = File::open(PASSWORD_FILE)?;
    let mut data: String = String::new();
    file.read_to_string(&mut data)?;
    Ok(serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
}

fn save_passwords(passwords: &Vec<PasswordEntry>) -> io::Result<()> {
    let data = serde_json::to_string(passwords)?;
    let mut file = File::create(PASSWORD_FILE)?;
    file.write_all(data.as_bytes())
}

pub(crate) fn list_passwords() -> io::Result<()> {
    let passwords = read_passwords()?;

    for entry in passwords {
        println!("URL: {}, Username: {}, Password: {}", entry.url, entry.username, entry.password);
    }

    Ok(())
}

pub(crate) fn search_password(query: String) -> io::Result<()> {
    let passwords = read_passwords()?;
    
    for entry in passwords {
        if entry.username.contains(&query) || entry.url.contains(&query) {
            println!("Found: URL: {}, Username: {}, Password: {}", entry.url, entry.username, entry.password);
        }
    }

    Ok(())
}

pub(crate) fn copy_to_clipboard(url: String) -> io::Result<()> {
    let passwords: Vec<PasswordEntry> = read_passwords()?;

    for entry in &passwords {
        if entry.url == url {
            let mut ctx: ClipboardContext = match ClipboardProvider::new() {
                Ok(ctx) => ctx,
                Err(e) => {
                    eprintln!("Erro ao acessar a área de transferência: {:?}", e);
                    return Err(io::Error::new(io::ErrorKind::Other, "Erro ao acessar a área de transferência"));
                }
            };

            if ctx.set_contents(entry.password.clone()).is_err() {
                eprintln!("Falha ao copiar para a área de transferência.");
                return Err(io::Error::new(io::ErrorKind::Other, "Falha ao copiar para a área de transferência"));
            }

            println!("Senha copiada para a área de transferência para a URL: {}", url);
            return Ok(());
        }
    }

    println!("URL não encontrada.");
    Ok(())
}