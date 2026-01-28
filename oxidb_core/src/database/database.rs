use std::string::ToString;
use crate::Table;

static DEFAULT_FILE_PATH: &str = "%AppData%/Oxidb/databases/";


struct OxiDataBase {
    tables: Vec<Table>,
}
