use crate::Table;

enum OxiBackend {
    Local(OxiLocalBackend),
    Remote(OxiRemoteBackend),
}

struct OxiLocalBackend { }

struct OxiRemoteBackend { }

pub trait OxiStorageBackend {
    fn load_table(&self, name: &str) -> Result<Table>;
    fn persist_row(&mut self, table: &str, row: &Row) -> Result<()>;
}

impl OxiStorageBackend for LocalBackend {
    fn load_table(&self, name: &str) -> Result<Table> { /* file IO */ }
    fn persist_row(&mut self, table: &str, row: &Row) -> Result<()> { /* fs write */ }
}

impl OxiStorageBackend for RemoteBackend {
    fn load_table(&self, name: &str) -> Result<Table> { /* HTTP */ }
    fn persist_row(&mut self, table: &str, row: &Row) -> Result<()> { /* HTTP */ }
}