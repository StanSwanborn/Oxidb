use std::collections::HashMap;

use crate::table::Table;

/// Temporary helper used during row insertion.
pub struct RowInserter<'a> {
    pub(crate) table: &'a mut Table,
    pub(crate) touched: HashMap<String, ()>,
}

impl<'a> RowInserter<'a> {
}