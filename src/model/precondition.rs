use super::*;
use crate::parser::*;
use std::collections::HashMap;

pub struct Precondition {
    name: String,
    expr: Expr,
    position: Option<Position>,
}

impl Precondition {
    pub fn new<S: Into<String>>(name: S, expr: Expr, position: Option<Position>) -> Self {
        let name = name.into();
        Self {
            name,
            expr,
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    pub fn position(&self) -> Option<Position> {
        self.position
    }

    //---------- Resolve ----------

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        self.expr.resolve_resource(map)
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        self.expr.resolve_state(map)
    }
}

impl ToLang for Precondition {
    fn to_lang(&self, model: &Model) -> String {
        format!("{}: {}", self.name, self.expr.to_lang(model))
    }
}

impl std::fmt::Display for Precondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
