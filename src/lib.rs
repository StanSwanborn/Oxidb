use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: u64,
    pub data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Table {
    pub name: String,
    pub records: HashMap<u64, Record>,
}

pub struct MiniDB {
    path: PathBuf,
    tables: HashMap<String, Table>,
}

impl MiniDB {
    pub fn new(path: &str) -> Self {
        if let Err(e) = fs::create_dir_all(path) {
            eprintln!("Failed to create directory: {}", e);
        }
        Self {
            path: PathBuf::from(path),
            tables: HashMap::new(),
        }
    }

    pub fn create_table(&mut self, name: &str) {
        self.tables.insert(name.to_string(), Table {
            name: name.to_string(),
            records: HashMap::new(),
        });
    }

    pub fn insert(&mut self, table: &str, record: Record) {
        if let Some(t) = self.tables.get_mut(table) {
            t.records.insert(record.id, record);
        }
    }

    pub fn save(&self) {
        for (name, table) in &self.tables {
            let file = self.path.join(format!("{}.json", name));
            fs::write(file, serde_json::to_string_pretty(&table).unwrap()).unwrap();
        }
    }

    pub fn load(&mut self) {
        for entry in fs::read_dir(&self.path).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().unwrap_or_default() == "json" {
                let data = fs::read_to_string(&path).unwrap();
                let table: Table = serde_json::from_str(&data).unwrap();
                self.tables.insert(table.name.clone(), table);
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let mut db = MiniDB::new("testdata/db1.json");
//         db.create_table("test_table");
//
//         let mut record = Record { id: 1, data: HashMap::new() };
//         record.data.insert("name".into(), "Stan".into());
//         record.data.insert("role".into(), "Admin".into());
//
//         db.insert("users", record);
//         db.save();
//
//         db.
//     }
// }
#[cfg(test)]
mod tests {
    use super::*; // gebruik alles uit het bovenliggende module
    use std::fs;
    use std::path::Path;

    fn cleanup_test_dir(path: &str) {
        if Path::new(path).exists() {
            fs::remove_dir_all(path).unwrap();
        }
    }

    #[test]
    fn test_create_new_database_directory() {
        let test_path = "./test_data_1";
        cleanup_test_dir(test_path);

        // Act
        let _db = MiniDB::new(test_path);

        // Assert
        assert!(Path::new(test_path).exists(), "Database directory should be created");

        cleanup_test_dir(test_path);
    }

    #[test]
    fn test_create_table_and_insert_record() {
        let test_path = "./test_data_2";
        cleanup_test_dir(test_path);

        let mut db = MiniDB::new(test_path);
        db.create_table("users");

        // Maak een record
        let mut record = Record { id: 1, data: HashMap::new() };
        record.data.insert("name".into(), "Stan".into());
        record.data.insert("role".into(), "Admin".into());

        db.insert("users", record.clone());

        // Controleer dat de table en record bestaan
        assert!(db.tables.contains_key("users"));
        assert_eq!(db.tables["users"].records.len(), 1);
        assert_eq!(db.tables["users"].records.get(&1).unwrap().data["name"], "Stan");

        cleanup_test_dir(test_path);
    }

    #[test]
    fn test_save_and_load_database() {
        let test_path = "./test_data_3";
        cleanup_test_dir(test_path);

        // Maak en vul database
        let mut db = MiniDB::new(test_path);
        db.create_table("products");

        let mut rec1 = Record { id: 10, data: HashMap::new() };
        rec1.data.insert("name".into(), "Laptop".into());
        rec1.data.insert("price".into(), "999".into());

        db.insert("products", rec1);
        db.save();

        // Controleer dat file is aangemaakt
        let file_path = format!("{}/products.json", test_path);
        assert!(Path::new(&file_path).exists(), "Table file should be written to disk");

        // Nieuwe DB inladen vanaf disk
        let mut db2 = MiniDB::new(test_path);
        db2.load();

        // Controleer dat data correct is hersteld
        let table = db2.tables.get("products").expect("Table 'products' should exist after load");
        let record = table.records.get(&10).expect("Record should exist after load");
        assert_eq!(record.data["name"], "Laptop");
        assert_eq!(record.data["price"], "999");

        cleanup_test_dir(test_path);
    }
}