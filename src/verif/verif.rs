use super::*;
use crate::model::*;

pub fn check_skillset(skillset: &Skillset) -> Vec<VError> {
    let mut v = vec![];
    // Event
    v.extend(check_events(skillset));
    // Skill
    v.extend(check_skills(skillset));
    //
    v
}

pub fn check_skill(skillset: &Skillset, skill: &Skill) -> Vec<VError> {
    let mut v = vec![];
    // Precondition
    v.extend(check_skill_preconditions(skillset, skill));
    // Start
    if let Some(e) = can_start_effects_fail(skillset, skill) {
        v.push(e)
    }
    // Invariant
    v.extend(check_skill_invariants(skillset, skill));
    // Terminate
    v.extend(check_skill_terminates(skillset, skill));
    //
    v
}

pub fn check_skills(skillset: &Skillset) -> Vec<VError> {
    skillset
        .skills()
        .iter()
        .map(|s| check_skill(skillset, s))
        .flatten()
        .collect()
}

pub fn check_model(model: &Model) -> Vec<VError> {
    model
        .skillsets()
        .iter()
        .map(|s| check_skillset(s))
        .flatten()
        .collect()
}
