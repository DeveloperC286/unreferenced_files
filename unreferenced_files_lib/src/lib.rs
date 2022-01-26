#![allow(clippy::result_unit_err)]
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;

pub mod file_path_variants;
mod file_path_variants_regexes;
pub mod filters;
pub mod search;
pub mod search_for;
mod utilities;

#[cfg(test)]
mod tests;
