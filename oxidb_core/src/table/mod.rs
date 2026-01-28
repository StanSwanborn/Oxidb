pub mod table;
mod row_builder;

pub use table::Table;

#[cfg(test)]
mod tests {
    mod shared;
    mod insert_tests;
    mod get_tests;
}