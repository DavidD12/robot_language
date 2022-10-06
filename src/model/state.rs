use super::*;
use crate::parser::Position;

#[derive(Clone, Copy, PartialEq)]
pub struct StateId(pub ResourceId, pub usize);
impl Id for StateId {
    fn empty() -> Self {
        Self(ResourceId::empty(), 0)
    }
}

pub struct State {
    id: StateId,
    name: String,
    position: Option<Position>,
}

impl State {
    pub fn empty<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = StateId::empty();
        let name = name.into();
        Self { id, name, position }
    }

    pub fn id(&self) -> StateId {
        self.id
    }

    pub(super) fn set_id(&mut self, id: StateId) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> Option<Position> {
        self.position
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
