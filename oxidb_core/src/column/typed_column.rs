use std::any::Any;

use crate::column::Column;

/// A column storing values of type T.
///
/// Example:
/// - T = i64
/// - T = bool
pub struct TypedColumn<T> {
    data: Vec<T>,
    default: T,
}

impl<T: Clone + 'static> TypedColumn<T> {
    pub fn new(default: T) -> Self {
        Self {
            data: Vec::new(),
            default,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    /// Get reference at index
    pub fn get(&self, index: usize) -> &T { &self.data[index] }

    /// Get mutable clone at index
    pub fn get_mut(&mut self, index: usize) -> T { self.data[index].clone() }

    /// Set value at index
    pub fn set(&mut self, index: usize, value: T) { self.data[index] = value; }
}

impl<T> Column for TypedColumn<T>
where
    T: Clone + 'static,
{
    fn len(&self) -> usize {
        self.data.len()
    }

    fn push_default(&mut self) {
        self.data.push(self.default.clone());
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_any(&self, index: usize) -> Box<dyn Any> {
        Box::new(self.data[index].clone())
    }
}