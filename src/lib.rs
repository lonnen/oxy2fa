extern crate ring;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use ring::hmac::Hmac;
use ring::sha1::Sha1;

const counterLen: usize = 20;
struct Keychain {
    file: String,
    data: Vec<u8>,
    keys: HashMap<String, Key>,
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

    // pub fn code(self, name: &String) -> &String {
    //     let key = self.keys.get(name).unwrap();
    //     key.code(self.data);
    //     &String::from("asdf")
    // }
}

struct Key {
    raw: Vec<u8>,
    digits: usize,
    offset: usize,
}

impl Key {
    /// Create an empty `Key`.
    ///
    /// # Examples
    ///
    /// ```
    /// use oxy2fa::Key;
    ///
    /// let k: Key = Key::new();
    ///
    ///
    /// ```
    pub fn new(raw: Vec<u8>, digits: usize, offset: usize) -> Option<Key> {
        // digits should be restricted to 6 and 9.3 digits (can be 0, 1, 2, 3) (log 31 b10)
        match digits {
            6 | 7 | 8 | 9 | 10 => (),
            _ => return None,
        }

        let hmac_key = vec![32; 0u8];

        let mut hmac = Hmac::new(Sha1::new(), hmac_key.as_slice());
        hmac.input(message);
        hmac.result().code().to_hex();

        Some(Key {
            raw: vec![],
            digits: 0,
            offset: 0,
        })
    }

    // pub fn code<'a>(&'a self, _n: Vec<u8>) -> &'a String {
    //     match self.offset {
    //         0 => &String::from("totp logic -> hotp logic"),
    //         _ => &String::from("hotp logic"),
    //     }
    // }
}

// fn H -> (sha-1)
// K: [u8]
// d: usize between (6, 9.3) // 9.3 because digit 10 can only be decimal (0,2,3) or log(31, base=10)
// C: counter of how many keys have been generated
//
// hmac
//
