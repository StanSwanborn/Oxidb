use crate::{ColumnSchema, ColumnType, Table, TableSchema};

pub fn shared_user_table_definition() -> Table {
    Table::new(TableSchema {
        name: "users_test_table".into(),
        columns: vec![
            ColumnSchema {
                name: "id".into(),
                column_type: ColumnType::I64
            },
            ColumnSchema {
                name: "name".into(),
                column_type: ColumnType::String
            },
            ColumnSchema {
                name: "created_at".into(),
                column_type: ColumnType::I64
            },
            ColumnSchema {
                name: "active".into(),
                column_type: ColumnType::Bool
            }
        ]
    })
}