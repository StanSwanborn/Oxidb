use time::OffsetDateTime;
use crate::column::{Column, TypedColumn};
use crate::table::tests::shared::shared_user_table_definition;

#[test]
fn rows_should_insert_correctly_into_table() {
    let mut user_table = shared_user_table_definition();
    let now = OffsetDateTime::now_utc().unix_timestamp();

    user_table.add_row(|row| {
        row.set_i64("id", 1);
        row.set_string("name", "Stan Swanborn");
        row.set_i64("created_at", now);
        row.set_bool("active", true);
    });

    let id_col = user_table.get_column("id")
        .as_any()
        .downcast_ref::<TypedColumn<i64>>()
        .unwrap();
    
    let name_col = user_table.get_column("name")
        .as_any()
        .downcast_ref::<TypedColumn<String>>()
        .unwrap();
    
    let created_col = user_table.get_column("created_at")
        .as_any()
        .downcast_ref::<TypedColumn<i64>>()
        .unwrap();
    
    let active_col = user_table.get_column("active")
        .as_any()
        .downcast_ref::<TypedColumn<bool>>()
        .unwrap();

    assert_eq!(id_col.len(), 1);
    assert_eq!(name_col.len(), 1);
    assert_eq!(created_col.len(), 1);
    assert_eq!(active_col.len(), 1);

    assert_eq!(*id_col.get(0), 1);
    assert_eq!(*name_col.get(0), "Stan Swanborn");
    assert_eq!(*created_col.get(0), now);
    assert_eq!(*active_col.get(0), true);
}

#[test]
#[should_panic(expected = "Column type mismatch")]
fn columns_should_reject_invalid_type_correctly()
{
    shared_user_table_definition().add_row(|row| {
        row.set_string("id", "this should fail!")
    });
}