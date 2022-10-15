use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SkillId(pub SkillsetId, pub usize);
impl Id for SkillId {
    fn empty() -> Self {
        Self(SkillsetId::empty(), 0)
    }
}

pub struct Skill {
    id: SkillId,
    name: String,
    parameters: Vec<SkillParameter>,
    inputs: Vec<Variable>,
    outputs: Vec<Variable>,
    preconditions: Vec<Precondition>,
    start: Vec<Effect>,
    invariants: Vec<Invariant>,
    progress: Option<Progress>,
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
            parameters: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            preconditions: Vec::new(),
            start: Vec::new(),
            invariants: Vec::new(),
            progress: None,
            interrupt: None,
            successes: Vec::new(),
            failures: Vec::new(),
            position,
        }
    }

    //---------- Parameter ----------

    pub fn parameter(&self) -> &Vec<SkillParameter> {
        &self.parameters
    }

    pub fn get_parameter(&self, id: SkillParameterId) -> Option<&SkillParameter> {
        let SkillParameterId(skill_id, parameter_id) = id;
        if self.id != skill_id {
            None
        } else {
            self.parameters.get(parameter_id)
        }
    }

    pub fn add_parameter(&mut self, mut parameter: SkillParameter) -> SkillParameterId {
        let id = SkillParameterId(self.id, self.parameters.len());
        parameter.set_id(id);
        self.parameters.push(parameter);
        id
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

    pub fn add_precondition(&mut self, mut precondition: Precondition) -> PreconditionId {
        let id = PreconditionId(self.id, self.preconditions.len());
        precondition.set_id(id);
        self.preconditions.push(precondition);
        id
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

    //---------- Progress ----------

    pub fn progress(&self) -> &Option<Progress> {
        &self.progress
    }

    pub fn set_progress(&mut self, progress: Progress) {
        self.progress = Some(progress)
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

    //---------- Duplicate ----------

    pub fn parameter_naming(&self) -> Vec<Naming> {
        self.parameters.iter().map(|x| x.naming()).collect()
    }
    pub fn input_naming(&self) -> Vec<Naming> {
        self.inputs
            .iter()
            .map(|x| (x.name().into(), x.position()))
            .collect()
    }
    pub fn output_naming(&self) -> Vec<Naming> {
        self.outputs
            .iter()
            .map(|x| (x.name().into(), x.position()))
            .collect()
    }
    pub fn precondition_naming(&self) -> Vec<Naming> {
        self.preconditions.iter().map(|x| x.naming()).collect()
    }
    pub fn invariant_naming(&self) -> Vec<Naming> {
        self.invariants.iter().map(|x| x.naming()).collect()
    }
    pub fn success_naming(&self) -> Vec<Naming> {
        self.successes.iter().map(|x| x.naming()).collect()
    }
    pub fn failure_naming(&self) -> Vec<Naming> {
        self.failures.iter().map(|x| x.naming()).collect()
    }

    pub fn duplicate(&self, model: &Model) -> Result<(), RlError> {
        let types = model.type_naming();

        // Parameter
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.parameter_naming().into_iter())
                .collect(),
        )?;
        // Input
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.input_naming().into_iter())
                .collect(),
        )?;
        // Output
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.output_naming().into_iter())
                .collect(),
        )?;
        // Precondition
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.precondition_naming().into_iter())
                .collect(),
        )?;
        // Invariant
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.invariant_naming().into_iter())
                .collect(),
        )?;
        // Success
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.success_naming().into_iter())
                .collect(),
        )?;
        // Failure
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.failure_naming().into_iter())
                .collect(),
        )?;

        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        // Parameter
        for x in self.parameters.iter_mut() {
            x.resolve_type(map)?;
        }
        // Input
        for x in self.inputs.iter_mut() {
            x.resolve_type(map)?;
        }
        // Output
        for x in self.outputs.iter_mut() {
            x.resolve_type(map)?;
        }
        // Progress
        if let Some(progress) = &mut self.progress {
            progress.resolve_type(map)?;
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

impl Named<SkillId> for Skill {
    fn id(&self) -> SkillId {
        self.id
    }
    fn set_id(&mut self, id: SkillId) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
        self.position
    }
}

impl ToLang for Skill {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = String::new();
        s.push_str(&format!("\t\t{} {{\n", self.name));
        // Parameter
        if !self.parameters.is_empty() {
            s.push_str("\t\t\tparameter {\n");
            for x in self.parameters.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
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
        // Invariant
        if !self.invariants.is_empty() {
            s.push_str("\t\t\tinvariant {\n");
            for x in self.invariants.iter() {
                s.push_str(&format!("\t\t\t\t{}", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Progress
        if let Some(progress) = &self.progress {
            s.push_str(&progress.to_lang(model));
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
