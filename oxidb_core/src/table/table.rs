use std::any::Any;
use std::collections::HashMap;

use crate::column::Column;
use crate::schema::{ColumnType, TableSchema};
use crate::column::typed_column::TypedColumn;
use crate::row::row::Row;
use crate::table::row_builder::RowBuilder;

/// A table instance holding actual data.
pub struct Table {
    pub schema: TableSchema,
    pub columns: HashMap<String, Box<dyn Column>>,
    row_count: usize,
}

impl Table {
    pub fn new(schema: TableSchema) -> Self {
        let mut columns= HashMap::new();

        // Create column storage based on schema
        for col in &schema.columns {
            let column: Box<dyn Column> = match col.column_type {
                ColumnType::I64 => Box::new(TypedColumn::<i64>::new(0)),
                ColumnType::F64 => Box::new(TypedColumn::<f64>::new(0.0)),
                ColumnType::Bool => Box::new(TypedColumn::<bool>::new(false)),
                ColumnType::Bytes => Box::new(TypedColumn::<Vec<u8>>::new(Vec::new())),
                ColumnType::String => Box::new(TypedColumn::<String>::new(String::new())),
            };

            columns.insert(col.name.clone(), column);
        }

        Self {
            schema,
            columns,
            row_count: 0,
        }
    }

    pub fn add_row<F>(&mut self, f: F) where F: FnOnce(&mut RowBuilder),
    {
        let mut builder = RowBuilder::new(self);
        f(&mut builder);
        builder.finish();

        self.row_count += 1;
    }

    /// get the row at index
    pub fn get_row(&self, index: usize) -> Row {
        if index >= self.row_count { panic!("Row index out of bounds"); }

        let mut row = Row::new();
        for(name, col) in &self.columns {
            row.push_boxed_value(name.clone(), col.get_any(index));
        }

        row
    }

    pub fn get_column(&self, name: &str) -> &Box<dyn Column> {
        self.columns.get(name).unwrap()
    }

    pub fn get_column_mut(&mut self, name: &str) -> &mut Box<dyn Column> {
        self.columns.get_mut(name).unwrap()
    }
}