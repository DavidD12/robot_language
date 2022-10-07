use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub struct InputId(pub SkillId, pub usize);
impl Id for InputId {
    fn empty() -> Self {
        Self(SkillId::empty(), 0)
    }
}

pub struct Input {
    id: InputId,
    name: String,
    rl_type: Reference<TypeId>,
    position: Option<Position>,
}

impl Input {
    pub fn empty<S: Into<String>>(
        name: S,
        rl_type: Reference<TypeId>,
        position: Option<Position>,
    ) -> Self {
        let id = InputId::empty();
        let name = name.into();
        Self {
            id,
            name,
            rl_type,
            position,
        }
    }

    pub fn id(&self) -> InputId {
        self.id
    }

    pub fn set_id(&mut self, id: InputId) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rl_type(&self) -> &Reference<TypeId> {
        &self.rl_type
    }

    pub fn set_type(&mut self, id: TypeId) {
        self.rl_type = Reference::Resolved(id);
    }

    pub fn position(&self) -> Option<Position> {
        self.position
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        match self.rl_type() {
            Reference::Unresolved(name, pos) => match map.get(name) {
                Some(id) => {
                    self.set_type(*id);
                    Ok(())
                }
                None => Err(RlError::Resolve {
                    element: format!("type '{}'", name),
                    position: *pos,
                }),
            },
            Reference::Resolved(_) => Ok(()),
        }
    }
}

impl ToLang for Input {
    fn to_lang(&self, model: &Model) -> String {
        format!("{}: {}", self.name, self.rl_type.to_lang(model))
    }
}

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}