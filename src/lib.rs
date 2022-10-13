#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub mod parser;
pub use parser::*;

pub mod model;
pub use model::*;

pub mod verif;
pub use verif::*;

pub fn process_file(model: &mut Model, filename: &str) -> Result<(), RlError> {
    // Parsing
    match parse_model_file(model, filename) {
        Ok(_) => info!("Parsing OK"),
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }
    // Duplicate
    match model.duplicate() {
        Ok(_) => info!("Duplicate OK"),
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }
    // Resolve
    match model.resolve() {
        Ok(_) => info!("Resolve OK"),
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }
    //
    Ok(())
}

pub fn load_model(file: &str) -> Result<Model, RlError> {
    let mut model = Model::empty();
    match process_file(&mut model, file) {
        Ok(_) => Ok(model),
        Err(e) => Err(e),
    }
}
