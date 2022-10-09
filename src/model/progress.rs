use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub struct ProgressId(pub SkillId, pub usize);
impl Id for ProgressId {
    fn empty() -> Self {
        Self(SkillId::empty(), 0)
    }
}

pub struct Progress {
    id: ProgressId,
    position: Option<Position>,
}

impl Progress {
    pub fn empty<S: Into<String>>(
        name: S,
        guard: Expr,
        effects: Vec<Effect>,
        position: Option<Position>,
    ) -> Self {
        let id = ProgressId::empty();
        let name = name.into();
        Self {
            id,
            name,
            guard,
            effects,
            position,
        }
    }

    pub fn id(&self) -> ProgressId {
        self.id
    }

    pub fn set_id(&mut self, id: ProgressId) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn guard(&self) -> &Expr {
        &self.guard
    }

    pub fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn position(&self) -> Option<Position> {
        self.position
    }

    //---------- Resolve ----------

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        self.guard.resolve_resource(map)?;
        for x in self.effects.iter_mut() {
            x.resolve_resource(map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        self.guard.resolve_state(map)?;
        for x in self.effects.iter_mut() {
            x.resolve_state(map)?;
        }
        Ok(())
    }
}

impl ToLang for Progress {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = format!("{} {{\n", self.name);
        // guard
        s.push_str(&format!("\t\t\t\t\tguard {}\n", self.guard.to_lang(model)));
        // Effects
        if !self.effects.is_empty() {
            s.push_str("\t\t\t\t\teffect {\n");
            for x in self.effects.iter() {
                s.push_str(&format!("\t\t\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t\t\t}\n");
        }
        //
        s.push_str("\t\t\t\t}\n");
        s
    }
}

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
