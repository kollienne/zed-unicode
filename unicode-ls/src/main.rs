use std::collections::HashMap;

use simple_completion_language_server::*;

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let snippets = include_str!("data.txt");
    let mut unicode = HashMap::new();
    for line in snippets.split("\n") {
        if line.is_empty() {
            continue;
        }
        let line = line.split(";").collect::<Vec<_>>();
        let [c, alias, ..] = line.as_slice() else {
            continue;
        };

        let Ok(c) = u32::from_str_radix(c, 16) else {
            continue;
        };

        let Ok(c) = char::try_from(c) else {
            continue;
        };

        unicode.insert(alias.to_string(), format!("{c}"));
    }

    println!("meow {:#?}", unicode);

    server::start(
        stdin,
        stdout,
        vec![],
        unicode,
        etcetera::home_dir().unwrap().to_str().unwrap().into(),
    )
    .await;
}
