use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub struct SkillId(pub SkillsetId, pub usize);
impl Id for SkillId {
    fn empty() -> Self {
        Self(SkillsetId::empty(), 0)
    }
}

pub struct Skill {
    id: SkillId,
    name: String,
    preconditions: Vec<Precondition>,
    position: Option<Position>,
}

impl Skill {
    pub fn empty<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = SkillId::empty();
        let name = name.into();
        Self {
            id,
            name,
            preconditions: Vec::new(),
            position,
        }
    }

    pub fn id(&self) -> SkillId {
        self.id
    }

    pub(super) fn set_id(&mut self, id: SkillId) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    //---------- Precondition ----------

    pub fn preconditions(&self) -> &Vec<Precondition> {
        &self.preconditions
    }

    pub fn add_precondition(&mut self, precondition: Precondition) {
        self.preconditions.push(precondition)
    }
    //---------- ----------

    pub fn position(&self) -> Option<Position> {
        self.position
    }

    //---------- Resolve ----------

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        for x in self.preconditions.iter_mut() {
            x.resolve_resource(map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        for x in self.preconditions.iter_mut() {
            x.resolve_state(map)?;
        }
        Ok(())
    }
}

impl ToLang for Skill {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = String::new();
        s.push_str(&format!("\t\t{} {{\n", self.name));
        // Precondition
        if !self.preconditions.is_empty() {
            s.push_str("\t\t\tprecondition {\n");
            for x in self.preconditions.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
        //
        s.push_str("\t\t}\n");
        s
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
