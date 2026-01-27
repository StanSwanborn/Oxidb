use time::OffsetDateTime;
use oxidb::*;
use oxidb::column::{Column, TypedColumn};

#[test]
fn insert_rows_into_table() {
    let mut user_table = Table::new(TableSchema {
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
    });

    let now = OffsetDateTime::now_utc().unix_timestamp();

    user_table.add_row(|row| {
        row.set_i64("id", 1);
        row.set_string("name", "Stan Swanborn");
        row.set_i64("created_at", now);
        row.set_bool("active", true);
    });

    // 3️⃣ Access columns and assert correctness
    let id_col = user_table.get_column("id").unwrap()
        .as_any()
        .downcast_ref::<TypedColumn<i64>>()
        .unwrap();
    let name_col = user_table.get_column("name").unwrap()
        .as_any()
        .downcast_ref::<TypedColumn<String>>()
        .unwrap();
    let created_col = user_table.get_column("created_at").unwrap()
        .as_any()
        .downcast_ref::<TypedColumn<i64>>()
        .unwrap();
    let active_col = user_table.get_column("active").unwrap()
        .as_any()
        .downcast_ref::<TypedColumn<bool>>()
        .unwrap();

    // 4️⃣ Check lengths
    assert_eq!(id_col.len(), 1);
    assert_eq!(name_col.len(), 1);
    assert_eq!(created_col.len(), 1);
    assert_eq!(active_col.len(), 1);

    // 5️⃣ Check values
    assert_eq!(id_col.get(0), 1);
    assert_eq!(name_col.get(0), "Stan Swanborn");
    assert_eq!(created_col.get(0), now);
    assert_eq!(active_col.get(0), true);
}