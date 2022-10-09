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
    inputs: Vec<Variable>,
    outputs: Vec<Variable>,
    preconditions: Vec<Precondition>,
    start: Vec<Effect>,
    invariants: Vec<Invariant>,
    interrupt: Option<Interrupt>,
    successes: Vec<Success>,
    failures: Vec<Failure>,
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
            invariants: Vec::new(),
            interrupt: None,
            successes: Vec::new(),
            failures: Vec::new(),
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

    pub fn inputs(&self) -> &Vec<Variable> {
        &self.inputs
    }

    pub fn add_input(&mut self, input: Variable) {
        self.inputs.push(input);
    }

    //---------- Output ----------

    pub fn outputs(&self) -> &Vec<Variable> {
        &self.outputs
    }

    pub fn add_output(&mut self, output: Variable) {
        self.outputs.push(output);
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

    //---------- Invariant ----------

    pub fn invariants(&self) -> &Vec<Invariant> {
        &self.invariants
    }

    pub fn add_invariant(&mut self, mut invariant: Invariant) -> InvariantId {
        let id = InvariantId(self.id, self.invariants.len());
        invariant.set_id(id);
        self.invariants.push(invariant);
        id
    }

    //---------- Interrupt ----------

    pub fn interrupt(&self) -> &Option<Interrupt> {
        &self.interrupt
    }

    pub fn set_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt = Some(interrupt);
    }

    //---------- Success ----------

    pub fn successes(&self) -> &Vec<Success> {
        &self.successes
    }

    pub fn add_success(&mut self, mut success: Success) -> SuccessId {
        let id = SuccessId(self.id, self.successes.len());
        success.set_id(id);
        self.successes.push(success);
        id
    }

    //---------- Failure ----------

    pub fn failures(&self) -> &Vec<Failure> {
        &self.failures
    }

    pub fn add_failure(&mut self, mut failure: Failure) -> FailureId {
        let id = FailureId(self.id, self.failures.len());
        failure.set_id(id);
        self.failures.push(failure);
        id
    }

    //---------- ----------

    pub fn position(&self) -> Option<Position> {
        self.position
    }

    //---------- Duplicate ----------

    pub fn names(&self) -> Vec<(String, Option<Position>)> {
        let mut v = Vec::new();
        // Input
        // for x in self.inputs.iter() {
        //     v.push((x.name().into(), x.position()));
        // }
        // Output
        // for x in self.outputs.iter() {
        //     v.push((x.name().into(), x.position()));
        // }
        // Precondition
        for x in self.preconditions.iter() {
            v.push((x.name().into(), x.position()));
        }
        // Invariant
        for x in self.invariants.iter() {
            v.push((x.name().into(), x.position()));
        }
        // Success
        for x in self.successes.iter() {
            v.push((x.name().into(), x.position()));
        }
        // Failure
        for x in self.failures.iter() {
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
        // Invariant
        for x in self.invariants.iter_mut() {
            x.resolve_resource(map)?;
        }
        // Interrupt
        if let Some(i) = &mut self.interrupt {
            i.resolve_resource(map)?;
        }
        // Success
        for x in self.successes.iter_mut() {
            x.resolve_resource(map)?;
        }
        // Failure
        for x in self.failures.iter_mut() {
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
        // Invariant
        for x in self.invariants.iter_mut() {
            x.resolve_state(map)?;
        }
        // Interrupt
        if let Some(i) = &mut self.interrupt {
            i.resolve_state(map)?;
        }
        // Success
        for x in self.successes.iter_mut() {
            x.resolve_state(map)?;
        }
        // Failure
        for x in self.failures.iter_mut() {
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
                s.push_str(&format!("\t\t\t\t{}", x.to_lang(model)))
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
        // Invariant
        if !self.invariants.is_empty() {
            s.push_str("\t\t\tinvariant {\n");
            for x in self.invariants.iter() {
                s.push_str(&format!("\t\t\t\t{}", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Interrupt
        if let Some(interrupt) = &self.interrupt {
            s.push_str(&interrupt.to_lang(model));
        }
        // Success
        if !self.successes.is_empty() {
            s.push_str("\t\t\tsuccess {\n");
            for x in self.successes.iter() {
                s.push_str(&format!("\t\t\t\t{}", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Failure
        if !self.failures.is_empty() {
            s.push_str("\t\t\tfailure {\n");
            for x in self.failures.iter() {
                s.push_str(&format!("\t\t\t\t{}", x.to_lang(model)))
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
