use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

/// A simple database to store generated graphs
/// and quickly check if one's already present there
pub struct Database {
    inner: HashMap<String, bool>,
}

const LABEL_TRUE: &str = "INTEGRAL";
const LABEL_FALSE: &str = "boring";

impl Database {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: bool) {
        self.inner.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<bool> {
        self.inner.get(key).cloned()
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for (key, value) in &self.inner {
            writeln!(writer, "{} {}", key, value)?;
        }
        writer.flush()?;
        Ok(())
    }

    pub fn new_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut db = Database::new();

        for line in reader.lines() {
            let line = line?;

            let mut split_line_iter = line.splitn(2, ' ');
            let key = match split_line_iter.next() {
                Some(key) => key,
                None => continue,
            };
            let value = match split_line_iter.next() {
                Some(value) => value,
                None => continue,
            };
            let value = match value {
                LABEL_TRUE => true,
                LABEL_FALSE => false,
                _ => continue,
            };
            db.inner.insert(key.to_string(), value);
        }
        Ok(db)
    }
}
