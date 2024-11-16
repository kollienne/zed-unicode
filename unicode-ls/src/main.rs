use std::collections::HashMap;

use simple_completion_language_server::*;
use snippets::Snippet;

fn capitalize(s: String) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v[0] = v[0].to_uppercase().next().unwrap();
    v.into_iter().collect()
}

fn get_prefix(s: &str) -> String {
    let s = s.replace("LATIN ", "");
    let s = s.replace("BALINESE ", "");
    let s = s.replace("GREEK ", "");
    let s = s.replace("TAI THAM HORA ", "");
    let s = s.replace("THAM COMBINING CRYPTOGRAMMIC ", "");
    let s = s.replace("TAI THAM SIGN ", "");
    let s = s.replace("TAI THAM VOWEL ", "");
    let s = s.replace(" ", "-");
    s
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let snippets = include_str!("data.txt");
    let mut unicode = vec![];
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

        let alias = alias.to_lowercase();
        let prefix = get_prefix(&alias);

        unicode.push(Snippet {
            scope: None,
            prefix,
            description: Some(capitalize(alias)),
            body: format!("{c}"),
        });
    }

    server::start(
        stdin,
        stdout,
        unicode,
        HashMap::new(),
        etcetera::home_dir().unwrap().to_str().unwrap().into(),
    )
    .await;
}
