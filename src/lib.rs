use std::collections::HashMap;

struct Keychain {
    file: String,
    data: Vec<String>,
    keys: HashMap<String, Key>
}

struct Key {
    raw: Vec<String>,
    digits: isize,
    offset: isize,
}

impl Keychain {
    pub fn new() -> Keychain {
        Keychain {
            file: String::from(""),
            data: vec![],
            keys: HashMap::<String, Key>::new(),
        }
    }

    pub fn add(name: String) {

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
}

