use std::collections::HashSet;
use crate::column::TypedColumn;
use crate::Table;

pub struct RowBuilder<'a> {
    table: &'a mut Table,
    touched: HashSet<String>,
}

impl<'a> RowBuilder<'a> {
    pub fn new(table: &'a mut Table) -> Self {
        Self {
            table,
            touched: HashSet::new(),
        }
    }

    pub fn set_i64(&mut self, column_name: &str, value: i64) {
        let col = self.table.columns.get_mut(column_name)
            .expect("Column not found");

        let typed = col.as_any_mut()
            .downcast_mut::<TypedColumn<i64>>()
            .expect("Column type mismatch");

        typed.push(value);
        self.touched.insert(column_name.to_string()); // HashSet works fine
    }

    pub fn set_f64(&mut self, column_name: &str, value: f64) {
        let col = self.table.columns.get_mut(column_name)
            .expect("Column not found");

        let typed = col.as_any_mut()
            .downcast_mut::<TypedColumn<f64>>()
            .expect("Column type mismatch");

        typed.push(value);
        self.touched.insert(column_name.to_string());
    }

    pub fn set_bool(&mut self, column_name: &str, value: bool) {
        let col = self.table.columns.get_mut(column_name)
            .expect("Column not found");

        let typed = col.as_any_mut()
            .downcast_mut::<TypedColumn<bool>>()
            .expect("Column type mismatch");

        typed.push(value);
        self.touched.insert(column_name.to_string());
    }

    pub fn set_string(&mut self, column_name: &str, value: &str) {
        let col = self.table.columns.get_mut(column_name)
            .expect("Column not found");

        let typed = col.as_any_mut()
            .downcast_mut::<TypedColumn<String>>()
            .expect("Column type mismatch");

        typed.push(value.to_string());
        self.touched.insert(column_name.to_string());
    }

    pub fn set_bytes(&mut self, column_name: &str, value: &[u8]) {
        let col = self.table.columns.get_mut(column_name)
            .expect("Column not found");

        let typed = col.as_any_mut()
            .downcast_mut::<TypedColumn<Vec<u8>>>()
            .expect("Column type mismatch");

        typed.push(value.to_vec());
        self.touched.insert(column_name.to_string());
    }

    pub fn finish(self) {
        // Push defaults for untouched columns
        for (name, col) in self.table.columns.iter_mut() {
            if !self.touched.contains(name) {
                col.push_default();
            }
        }
    }
}