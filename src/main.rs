use anyhow::Result;
use std::{collections::HashMap, ffi::OsStr, fs::{read, read_dir}};

#[derive(Debug)]
struct Config<'a> {
    forbidden_paths: Vec<Option<&'a str>>,
    amount_of_files: HashMap<String, usize>,
    filetypes_blacklist: Vec<&'a str>,
    total_bytes: usize
}

fn main() -> Result<()> {
    let path: &str = &(std::env::var("HOME")? + "/Documents/");
    let mut config = Config { 
        forbidden_paths: vec![Some("target"), Some("node_modules"), Some(".git"), Some(".gitignore"), Some(".env"), Some("commit"), Some("Debug"), Some("NoMachine"), Some("'Obsidian Vault'")], 
        amount_of_files: HashMap::new(),
        filetypes_blacklist: vec!["json", "csproj", "zip", "png", "jpg", "targets", "docx", "txt",
        "props", "out", "cache", "sql", "md", "ttf", "gif", "yaml",
        "lockb", "lock", "toml", "mod", "svg", "map", "sum"],
        total_bytes: 0
    };
    println!("{}", path);
    let _ = recursively_read_dir(path, &mut config);
    println!("{:#?} {:?}", config.total_bytes, config.amount_of_files);
    Ok(())
}

fn recursively_read_dir(path: &str, config: &mut Config) -> Result<()>{
    for entry in read_dir(path)?  {
        let finding = entry?;
        if finding.path().is_dir() {
            if !config.forbidden_paths.contains(&finding.path().file_name().unwrap().to_str()) &&
                &finding.path().file_name().unwrap().to_str().unwrap().chars().next() != &Some('.') { 
                let _ = recursively_read_dir(finding.path().to_str().expect("Skill issue"), config);
            }
        } else if !config.forbidden_paths.contains(&finding.path().file_name().unwrap().to_str()) &&
                &finding.path().file_name().unwrap().to_str().unwrap().chars().next() != &Some('.') &&
                finding.path().file_name().unwrap().to_str().unwrap().contains(".") {

            let extension = finding.path().extension().expect("Extension is none").to_owned().to_str().unwrap().to_owned();

            if !config.filetypes_blacklist.contains(&extension.as_str()) {
                let amount = read(finding.path())?.len();
                config.total_bytes += amount;
                config.amount_of_files.insert(extension.clone(), amount + 
                                              if config.amount_of_files.contains_key(&extension.to_string()) { 
                                                  config.amount_of_files.get(&extension.to_string()).expect("not found").to_owned() 
                                              } else {
                                                  0 
                                              });
            }
        }
    }
    Ok(())
}
// w oparciu o baze danych do tabeli farby przygotuj formularz zamowienia. uzytkownik wybiera
// rodzaj farby i jej ilosc skrypt sprawdza dostepnosc jesli farba jest dostepna nastepuje akup a z
// bazy odejmuje sie ilosc zakupionej farby. jesli jest niedostepna albo mniej niz chce uzytkownik
// kupic, informujemy o braku dostepnosci
