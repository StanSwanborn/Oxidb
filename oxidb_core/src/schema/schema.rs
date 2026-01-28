/// The supported column types in our database.
/// This is *metadata*, not the storage itself.
#[derive(Debug, Clone, Copy)]
pub enum ColumnType {
    I64,
    F64,
    Bool,
    Bytes,
    String
}

/// One column definition in a table.
#[derive(Debug, Clone)]
pub struct ColumnSchema {
    pub name: String,
    pub column_type: ColumnType,
}

/// The full schema of a table.
#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: String,
    pub columns: Vec<ColumnSchema>,
}