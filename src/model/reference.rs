use super::*;
use crate::{Position, ToLang};

pub trait Id: Clone + Copy + PartialEq {
    fn empty() -> Self;
}

#[derive(Clone)]
pub enum Reference<T: Id> {
    Unresolved(String, Option<Position>),
    Resolved(T),
}

impl ToLang for Reference<TypeId> {
    fn to_lang(&self, model: &crate::Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get_type(*id).unwrap()),
        }
    }
}

impl ToLang for Reference<StateId> {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get_state(*id).unwrap()),
        }
    }
}
