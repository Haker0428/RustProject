//! This is documentation for the 'csv_demo' lib crate
//! Usage:
//! ```
//! use csv_demo::{
//!     Opt,
//!     {load_csv, write_csv},
//!     replace_column,
//! }
mod opt;
mod err;
mod core;

pub use self::opt::Opt;
pub use self::core:: {
    read::{load_csv, write_csv},
    write::replace_colunmn,
};