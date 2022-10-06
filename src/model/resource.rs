use super::*;
use crate::parser::Position;

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
    // transitions: Vec<Transition>,
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

    pub fn set_initial(&mut self, state: Reference<StateId>) {
        self.initial = state;
    }

    pub fn position(&self) -> Option<Position> {
        self.position
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
        s.push_str("\t\t}\n");
        s
    }
}
