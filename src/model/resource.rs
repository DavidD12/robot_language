use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
pub struct ResourceId(pub SkillsetId, pub usize);
impl Id for ResourceId {
    fn empty() -> Self {
        Self(SkillsetId::empty(), 0)
    }
}

pub struct Resource {
    id: ResourceId,
    name: String,
    states: Vec<State>,
    initial: Reference<StateId>,
    transitions: Transitions,
    position: Option<Position>,
}

impl Resource {
    pub fn empty<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = ResourceId::empty();
        let name = name.into();
        Self {
            id,
            name,
            states: Vec::new(),
            initial: Reference::Unresolved("?".into(), None),
            transitions: Transitions::All,
            position,
        }
    }

    pub fn id(&self) -> ResourceId {
        self.id
    }

    pub(super) fn set_id(&mut self, id: ResourceId) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn states(&self) -> &Vec<State> {
        &self.states
    }

    pub fn add_state(&mut self, mut state: State) -> StateId {
        let id = StateId(self.id, self.states.len());
        state.set_id(id);
        self.states.push(state);
        id
    }

    pub fn get_state(&self, id: StateId) -> Option<&State> {
        let StateId(resource_id, state_id) = id;
        if self.id != resource_id {
            None
        } else {
            self.states.get(state_id)
        }
    }

    pub fn find_state(&self, name: &str) -> Option<StateId> {
        for x in self.states.iter() {
            if x.name() == name {
                return Some(x.id());
            }
        }
        None
    }

    pub fn set_initial(&mut self, state: Reference<StateId>) {
        self.initial = state;
    }

    pub fn transitions(&self) -> &Transitions {
        &self.transitions
    }

    pub fn set_transitions(&mut self, transitions: Transitions) {
        self.transitions = transitions;
    }

    pub fn position(&self) -> Option<Position> {
        self.position
    }

    //---------- Duplicate ----------

    pub fn duplicate(&self) -> Result<(), RlError> {
        for (i, x) in self.states.iter().enumerate() {
            for y in self.states.iter().skip(i + 1) {
                if x.name() == y.name() {
                    return Err(RlError::Duplicate {
                        name: x.name().into(),
                        first: x.position(),
                        second: y.position(),
                    });
                }
            }
        }
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_state(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        // TODO
        Ok(())
    }
}

impl ToLang for Resource {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = String::new();
        s.push_str(&format!("\t\t{} {{\n", self.name));
        // state
        s.push_str("\t\t\tstate {");
        for x in self.states.iter() {
            s.push_str(&format!(" {}", x));
        }
        s.push_str(" }\n");
        // initial
        s.push_str(&format!("\t\t\tinitial {}\n", self.initial.to_lang(model)));
        // transitions
        s.push_str(&self.transitions.to_lang(model));
        //
        s.push_str("\t\t}\n");
        s
    }
}
