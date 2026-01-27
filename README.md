** very early stage, under construction **

This library aims to provide a lightweight, SQLite-like database implemented entirely in Rust, designed for high-performance access on moderate datasets (up to a few million rows).

Key features:

* Columnar storage: Each column is stored independently, enabling fast lookups and efficient memory usage.

* Typed columns: Supports primitive types (i64, f64, bool, String, Vec<u8>), with type-safe access.

* Rust-native API: No SQL required — fully programmatic access using idiomatic Rust.

* Lean and minimal: Focused on speed, simplicity, and low overhead.

This library is intended as a standalone Rust crate for embedded, in-memory, or file-backed storage where you want the performance and ergonomics of a columnar database without external dependencies.
