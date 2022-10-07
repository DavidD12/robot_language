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
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    preconditions: Vec<Precondition>,
    start: Vec<Effect>,
    position: Option<Position>,
}

impl Skill {
    pub fn empty<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = SkillId::empty();
        let name = name.into();
        Self {
            id,
            name,
            inputs: Vec::new(),
            outputs: Vec::new(),
            preconditions: Vec::new(),
            start: Vec::new(),
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

    //---------- Input ----------

    pub fn inputs(&self) -> &Vec<Input> {
        &self.inputs
    }

    pub fn add_input(&mut self, mut input: Input) -> InputId {
        let id = InputId(self.id, self.inputs.len());
        input.set_id(id);
        self.inputs.push(input);
        id
    }

    //---------- Output ----------

    pub fn outputs(&self) -> &Vec<Output> {
        &self.outputs
    }

    pub fn add_output(&mut self, mut output: Output) -> OutputId {
        let id = OutputId(self.id, self.outputs.len());
        output.set_id(id);
        self.outputs.push(output);
        id
    }

    //---------- Precondition ----------

    pub fn preconditions(&self) -> &Vec<Precondition> {
        &self.preconditions
    }

    pub fn add_precondition(&mut self, precondition: Precondition) {
        self.preconditions.push(precondition)
    }

    //---------- Start ----------

    pub fn start(&self) -> &Vec<Effect> {
        &self.start
    }

    pub fn set_start(&mut self, effects: Vec<Effect>) {
        self.start = effects;
    }

    //---------- ----------

    pub fn position(&self) -> Option<Position> {
        self.position
    }

    //---------- Duplicate ----------

    pub fn names(&self) -> Vec<(String, Option<Position>)> {
        let mut v = Vec::new();
        // Input
        for x in self.inputs.iter() {
            v.push((x.name().into(), x.position()));
        }
        // Output
        for x in self.outputs.iter() {
            v.push((x.name().into(), x.position()));
        }
        // Precondition
        for x in self.preconditions.iter() {
            v.push((x.name().into(), x.position()));
        }
        //
        v
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        // Input
        for x in self.inputs.iter_mut() {
            x.resolve_type(map)?;
        }
        // Output
        for x in self.outputs.iter_mut() {
            x.resolve_type(map)?;
        }
        Ok(())
    }

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        // Precondition
        for x in self.preconditions.iter_mut() {
            x.resolve_resource(map)?;
        }
        // Start
        for x in self.start.iter_mut() {
            x.resolve_resource(map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        // Precondition
        for x in self.preconditions.iter_mut() {
            x.resolve_state(map)?;
        }
        // Start
        for x in self.start.iter_mut() {
            x.resolve_state(map)?;
        }
        Ok(())
    }
}

impl ToLang for Skill {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = String::new();
        s.push_str(&format!("\t\t{} {{\n", self.name));
        // Input
        if !self.inputs.is_empty() {
            s.push_str("\t\t\tinput {\n");
            for x in self.inputs.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Output
        if !self.inputs.is_empty() {
            s.push_str("\t\t\toutput {\n");
            for x in self.outputs.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Precondition
        if !self.preconditions.is_empty() {
            s.push_str("\t\t\tprecondition {\n");
            for x in self.preconditions.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Start
        if !self.start.is_empty() {
            s.push_str("\t\t\tstart {\n");
            for x in self.start.iter() {
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
