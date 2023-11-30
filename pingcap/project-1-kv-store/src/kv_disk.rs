use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::{self, File}, io::{self, Read, Write}, path::Path};
use serde_json;
#[derive(Serialize, Deserialize, Default)]
pub struct KvStoreDisk {
    map: HashMap<String, String>,
}

impl KvStoreDisk {
    pub fn new() -> io::Result<KvStoreDisk> {
        let path = Path::new("kvs.json");
        if path.exists() {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            let map = serde_json::from_str(&contents)?;
            Ok(KvStoreDisk { map })
        } else {
            Ok(KvStoreDisk::default())
        }
    }

    pub fn set(&mut self, key: String, value: String) -> io::Result<()> {
        self.map.insert(key, value);
        self.save_to_disk()
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) -> io::Result<Option<String>> {
        let result = self.map.remove(&key);
        self.save_to_disk()?;
        Ok(result)
    }

    fn save_to_disk(&self) -> io::Result<()> {
        let serialized = serde_json::to_string(&self.map)?;
        fs::write("kvs.json", serialized)?;
        Ok(())
    }
}
