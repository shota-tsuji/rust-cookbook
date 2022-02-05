use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

#[derive(Deserialize, Debug)]
struct Item {
    foo: u64,
    bar: u64,
}

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

fn main() {
    let item_string = match read_file("./Item.toml".to_owned()) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    let items_table: HashMap<String, Vec<Item>> = toml::from_str(item_string.as_str()).unwrap();
    let items: &[Item] = &items_table["items"];

    println!("{:?}", items_table);
    println!("{:?}", items);
}
