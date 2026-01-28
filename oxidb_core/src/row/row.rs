use std::any::Any;
use std::collections::HashMap;
use crate::column::Column;

pub struct Row {
    pub column_values: HashMap<String, Box<dyn Any>>, // one value per column
}

impl Row {
    pub fn new() -> Self {
        Self { column_values: HashMap::new() }
    }

    /// Add a value of any type
    pub fn push_boxed_value(&mut self, name: String, value: Box<dyn Any>) {
        self.column_values.insert(name, value);
    }

    // /// Retrieve a value as a specific type
    // pub fn get<T: 'static>(&self, index: usize) -> Option<&T> {
    //     self.column_values.get(index)?.downcast_ref::<T>()
    // }
}