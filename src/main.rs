use anyhow::Result;
use colored::{self, Colorize};
use serde_json;
use std::{
    collections::HashMap,
    fs::{read, read_dir, read_to_string},
};

#[derive(Debug)]
struct Config<'a> {
    forbidden_paths: Vec<Option<&'a str>>,
    amount_of_files: HashMap<String, usize>,
    filetypes_blacklist: Vec<&'a str>,
    total_bytes: usize,
}

#[derive(Debug)]
struct Language {
    ext: String,
    name: String,
    col: colored::CustomColor,
    percentage: u8,
}

fn main() -> Result<()> {
    let path: &str = &(std::env::var("HOME")? + "/Documents/");
    let mut config = Config {
        forbidden_paths: vec![
            Some("target"),
            Some("node_modules"),
            Some(".git"),
            Some(".gitignore"),
            Some(".env"),
            Some("commit"),
            Some("Debug"),
            Some("NoMachine"),
            Some("'Obsidian Vault'"),
        ],
        amount_of_files: HashMap::new(),
        filetypes_blacklist: vec![
            "json", "csproj", "zip", "png", "jpg", "targets", "docx", "txt", "props", "out",
            "cache", "sql", "md", "ttf", "gif", "yaml", "lockb", "lock", "toml", "mod", "svg",
            "map", "sum",
        ],
        total_bytes: 0,
    };
    let _ = recursively_read_dir(path, &mut config);
    println!("{:#?} {:?}", config.total_bytes, config.amount_of_files);

    let items = read_to_string("items.json")?;
    let b: serde_json::Value = serde_json::from_str(&items)?;
    let mut languages: Vec<Language> = vec![];

    if let Some(langs) = b.as_array() {
        for c in langs {
            languages.push(Language {
                ext: c["ext"].to_string().replace("\"", ""),
                name: c["name"].to_string().replace("\"", ""),
                col: colored::CustomColor {
                    r: c["col"].as_object().unwrap()["r"].as_u64().unwrap() as u8,
                    g: c["col"].as_object().unwrap()["g"].as_u64().unwrap() as u8,
                    b: c["col"].as_object().unwrap()["b"].as_u64().unwrap() as u8,
                },
                percentage: (*config.amount_of_files.entry(c["ext"].to_string().replace("\"", "")).or_insert(0) as f64
                    / config.total_bytes as f64
                    * 100.0)
                    .round() as u8,
            })
        }
    }

    println!("{:#?}", languages);

    print_bar(languages);

    Ok(())
}

fn print_bar(mut languages: Vec<Language>) {
    languages.sort_by(|a, b| {b.percentage.cmp(&a.percentage) });
   for lang in &languages {
        for i in 0..lang.percentage {
            let x = "x".custom_color(lang.col);
            print!("{x}");
        }
   } 
   for lang in &languages[0..10] {
        println!("{:?}", lang)
   }
}

fn recursively_read_dir(path: &str, config: &mut Config) -> Result<()> {
    for entry in read_dir(path)? {
        let finding = entry?;
        if finding.path().is_dir() {
            if !config
                .forbidden_paths
                .contains(&finding.path().file_name().unwrap().to_str())
                && &finding
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .chars()
                    .next()
                    != &Some('.')
            {
                let _ = recursively_read_dir(finding.path().to_str().expect("Skill issue"), config);
            }
        } else if !config
            .forbidden_paths
            .contains(&finding.path().file_name().unwrap().to_str())
            && &finding
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .chars()
                .next()
                != &Some('.')
            && finding
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains(".")
        {
            let extension = finding
                .path()
                .extension()
                .expect("Extension is none")
                .to_owned()
                .to_str()
                .unwrap()
                .to_owned();

            if !config.filetypes_blacklist.contains(&extension.as_str()) {
                let amount = read(finding.path())?.len();
                config.total_bytes += amount;
                config.amount_of_files.insert(
                    extension.clone(),
                    amount
                        + if config.amount_of_files.contains_key(&extension.to_string()) {
                            config
                                .amount_of_files
                                .get(&extension.to_string())
                                .expect("not found")
                                .to_owned()
                        } else {
                            0
                        },
                );
            }
        }
    }
    Ok(())
}
