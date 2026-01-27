use std::any::Any;
use std::io::Write;

/// Trait implemented by all column storage types.
///
/// This allows us to:
/// - store different column types together
/// - call common methods without knowing T
pub trait Column {
    /// Number of rows in this column
    fn len(&self) -> usize;

    /// Push a default value (used when inserting a row
    /// that doesn't specify this column)
    fn push_default(&mut self);

    /// Serialize the column to disk
    fn serialize(&self, writer: &mut dyn Write);

    /// Used for downcasting (advanced, optional for now)
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}