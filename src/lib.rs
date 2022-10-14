#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub mod model;
pub mod parser;
pub mod verif;

pub fn process_file(model: &mut model::Model, filename: &str) -> Result<(), parser::RlError> {
    // Parsing
    match parser::parse_model_file(model, filename) {
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

pub fn load_model(file: &str) -> Result<model::Model, parser::RlError> {
    let mut model = model::Model::empty();
    match process_file(&mut model, file) {
        Ok(_) => Ok(model),
        Err(e) => Err(e),
    }
}
