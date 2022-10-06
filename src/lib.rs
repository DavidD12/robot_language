#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub mod parser;
pub use parser::*;

pub mod model;
pub use model::*;

pub fn process_file(model: &mut Model, filename: &str) -> Result<(), RlError> {
    // Parsing
    match parse_model_file(model, filename) {
        Ok(_) => info!("Parsing OK"),
        e => return e,
    }
    Ok(())
}
