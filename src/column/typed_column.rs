use std::any::Any;
use std::io::Write;

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

    pub fn get(&self, index: usize) -> T {
        self.data[index].clone()
    }
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

    fn serialize(&self, writer: &mut dyn Write) {
        // VERY naive binary serialization for now
        // This works only for simple POD types
        let bytes = unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr() as *const u8,
                self.data.len() * std::mem::size_of::<T>(),
            )
        };

        let _ = writer.write_all(bytes);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}