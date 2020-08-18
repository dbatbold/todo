use std::collections::HashMap;
use std::fs;

pub struct Config {
    fpath: String,
    map: HashMap<String, String>,
}

pub fn load_file(file_path: &str) -> Config {
    let content = fs::read_to_string(file_path).unwrap();
    load(content, file_path)
}

fn load(content: String, file_path: &str) -> Config {
    let mut map = HashMap::new();

    content
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .for_each(|line| {
            let pair: Vec<&str> = (*line).splitn(2, " ").collect();
            let key = pair[0].to_string();
            if key.find("#") == Some(0) {
                // skip comments
                return;
            }
            if pair.len() < 2 {
                if pair.len() == 1 && !key.is_empty() {
                    panic!(
                        "Config entry '{}' is missing a value in '{}'.",
                        &key, file_path
                    );
                }
                return;
            }
            let value = pair[1].to_string();
            if key.is_empty() {
                panic!(
                    "Config value '{}' is missing a key in '{}'.",
                    value, file_path
                );
            }
            if map.get(&key).is_some() {
                panic!(
                    "Duplicate key '{}' found in '{}'.",
                    &key, file_path
                );
            }
            map.insert(key, value);
        });

    Config {
        fpath: file_path.to_string(),
        map,
    }
}

impl Config {
    pub fn get(&self, key: &str) -> String {
        match self.map.get(key) {
            Some(v) => v.to_string(),
            None => String::new(),
        }
    }

    pub fn get_or(&self, key: &str, default: &str) -> String {
        match self.map.get(key) {
            Some(v) => v.to_string(),
            None => default.to_string(),
        }
    }

    pub fn get_i32_or(&self, key: &str, default: i32) -> i32 {
        match self.map.get(key) {
            Some(v) => v.parse::<i32>().expect(&format!(
                "Failed to parse '{}' as i32",
                v
            )),
            None => default,
        }
    }

    pub fn get_must(&self, key: &str) -> String {
        match self.map.get(key) {
            Some(v) => v.to_string(),
            None => panic!(
                "'{}' key is missing from config in '{}'.",
                key, self.fpath
            ),
        }
    }
}

#[test]
fn test_load_file() {
    let config = load_file("./todo-test.conf");
    assert_ne!(&config.get("postgres.constr"), "");
    assert_eq!(&config.get("missing"), "");
    assert_eq!(&config.get_or("missing", "default"), "default");
}

#[test]
#[should_panic(expected = "'missing' key is missing from config in './todo-test.conf'.")]
fn test_get_must() {
    let config = load_file("./todo-test.conf");
    config.get_must("missing");
}

#[test]
#[should_panic(expected = "Duplicate key 'dup' found in 'test.conf'.")]
fn test_duplicate_keys() {
    let content = "dup 1\ndup 2\n".to_string();
    load(content, "test.conf");
}
