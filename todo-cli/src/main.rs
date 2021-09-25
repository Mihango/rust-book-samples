#![allow(unused)]

use std::collections::HashMap;
use std::error;
use std::fs;
use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occured: {}", why),
            },
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    // reads string
    // fn new() -> Result<Todo, io::Error> {
    //     let mut f = std::fs::OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .read(true)
    //         .open("db.txt")?;

    //     let mut content = String::new();
    //     f.read_to_string(&mut content)?; // have to use std::io::Read to bring read_to_string to scope

    //     let map = content
    //         .lines()
    //         .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>()) // splitting string at tabs and collecting to a collection
    //         .map(|v| (v[0], v[1])) // converting collection to tuple
    //         .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap())) // converting to a hash map -- use std::str::FromStr to bring from_str to scope
    //         .collect();

    //     Ok(Todo { map })
    // }

    // reads json
    fn new() -> Result<Todo, io::Error> {
        let f = fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("db.json")?;

        // serialize json as hash map
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    // save for string
    // fn save(self) -> Result<(), io::Error> {
    //     let mut content = String::new();

    //     for (k, v) in self.map {
    //         let record = format!("{}\t{}\n", k, v);
    //         content.push_str(&record);
    //     }

    //     fs::write("db.txt", content)
    // }

    fn save(self) -> Result<(), Box<dyn error::Error>> {
        // a Box is an allocation
        // in memory
        // open db.json
        let f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        // write to file with serde
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false), //* deferences the value and set it to false
            None => None,
        }
    }
}
