use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::io;

mod functions;

#[derive(Serialize, Deserialize, Debug)]
struct PasswordEntry {
    username: String,
    password: String,
    url: String,
}

fn main() -> io::Result<()> {
    let matches = Command::new("Mini Password Manager")
        .version("1.0")
        .author("Seu Nome")
        .about("Gerencia suas senhas")
        .subcommand(
            Command::new("add")
                .about("Adiciona uma nova senha")
                .arg(Arg::new("username").required(true).help("Nome de usuário"))
                .arg(Arg::new("password").required(true).help("Senha"))
                .arg(Arg::new("url").required(true).help("URL associada")),
        )
        .subcommand(Command::new("list").about("Lista todas as senhas salvas"))
        .subcommand(
            Command::new("search")
                .about("Busca por uma senha específica")
                .arg(Arg::new("query").required(true).help("Nome de usuário ou URL para busca")),
        )
        .subcommand(
            Command::new("clip")
                    .about("Copiar senha para o clipboard")
                    .arg(Arg::new("url").required(true).help("URL associada")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            let username = sub_matches.get_one::<String>("username").unwrap().to_string();
            let password = sub_matches.get_one::<String>("password").unwrap().to_string();
            let url = sub_matches.get_one::<String>("url").unwrap().to_string();

            functions::add_password(username, password, url)?
        },
        Some(("list", _)) => {
            functions::list_passwords()?
        },
        Some(("search", sub_matches)) => {
            let query: String = sub_matches.get_one::<String>("query").unwrap().to_string();
            functions::search_password(query)?
        },
        Some(("clip", sub_matches)) => {
            let url: String = sub_matches.get_one::<String>("url").unwrap().to_string();
            functions::copy_to_clipboard(url)?
        },
        _ => {} 
    }

    Ok(())
}
