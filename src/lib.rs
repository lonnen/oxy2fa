use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

const counterLen: usize = 20;
struct Keychain {
    file: String,
    data: Vec<bool>,
    keys: HashMap<String, Key>,
}

struct Key {
    raw: Vec<bool>,
    digits: usize,
    offset: usize,
}

impl Keychain {
    pub fn new() -> Keychain {
        Keychain {
            file: String::from(""),
            data: vec![],
            keys: HashMap::<String, Key>::new(),
        }
    }

    pub fn add(mut self, name: String, key: Key) -> Keychain {
        self.keys.insert(name, key);
        self
    }

    pub fn code(self, name: &String) -> &String {
        let key = self.keys.get(name).unwrap();
        key.code(self.data);
        &String::from("asdf")
    }

}

impl Key {
    pub fn new() -> Key {
        Key {
            raw: vec![],
            digits: 0,
            offset: 0,
        }
    }

    pub fn code<'a>(&'a self, _n: Vec<bool>) -> &'a String {
        match self.offset {
            0 => String::from("totp logic -> hotp logic"),
            _ => String::from("hotp logic"),
        }
    }
}

