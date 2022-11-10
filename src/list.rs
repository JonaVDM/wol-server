use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::fs;
use std::fs::OpenOptions;
use uuid::Uuid;

#[derive(Debug)]
pub struct NotFoundError {}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "item not found")
    }
}

impl error::Error for NotFoundError {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub mac: String,
}

pub fn get_item(id: String) -> Result<Item, NotFoundError> {
    let items = get_items()
        .into_iter()
        .filter(|x| x.id == id)
        .collect::<Vec<Item>>();

    if items.len() == 0 {
        Err(NotFoundError {})
    } else {
        let item: &Item = items.get(0).unwrap();
        Ok(Item::clone(item))
    }
}

pub fn get_items() -> Vec<Item> {
    match fs::File::open("computers.yml") {
        Ok(f) => serde_yaml::from_reader(f).expect("error in yaml"),
        Err(_) => vec![],
    }
}

pub fn delete_item(id: String) {
    let items = get_items()
        .into_iter()
        .filter(|x| x.id != id)
        .collect::<Vec<Item>>();

    let file = fs::File::create("computers.yml").unwrap();

    serde_yaml::to_writer(file, &items).expect("Could not save");
}

pub fn add_item(name: String, mac: String) -> Result<(), ()> {
    // let file = fs::File::create("computers.yml").unwrap();
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("computers.yml")
        .unwrap();

    match serde_yaml::to_writer(
        file,
        &vec![Item {
            id: Uuid::new_v4().to_string(),
            name,
            mac,
        }],
    ) {
        Ok(()) => Ok(()),
        Err(_) => Err(()),
    }
}
