use super::*;
use crate::model::*;

pub fn precondition_can_succeed(
    skillset: &Skillset,
    precondition: &Precondition,
) -> Option<VError> {
    let PreconditionId(skill_id, _) = precondition.id();
    let skill = skillset.get(skill_id).unwrap();
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(false);
    // Add preconditions until 'precondition'
    for pre in skill.preconditions().iter() {
        solver.assert(&smt.to_bool(pre.expr(), false));
        if pre.id() == precondition.id() {
            break;
        }
    }
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => Some(VError::SkillPreconditionCantSucceed(precondition.id())),
        z3::SatResult::Unknown => panic!("unknown"),
        z3::SatResult::Sat => None,
    }
}

pub fn precondition_can_fail(skillset: &Skillset, precondition: &Precondition) -> Option<VError> {
    let PreconditionId(skill_id, _) = precondition.id();
    let skill = skillset.get(skill_id).unwrap();
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(false);
    // Add preconditions before 'precondition'
    for pre in skill.preconditions().iter() {
        if pre.id() == precondition.id() {
            break;
        }
        solver.assert(&smt.to_bool(pre.expr(), false));
    }
    solver.assert(&z3::ast::Bool::not(
        &smt.to_bool(precondition.expr(), false),
    ));
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => Some(VError::SkillPreconditionCantFail(precondition.id())),
        z3::SatResult::Unknown => panic!("unknown"),
        z3::SatResult::Sat => None,
    }
}

pub fn check_precondition(skillset: &Skillset, precondition: &Precondition) -> Vec<VError> {
    let mut v = vec![];
    if let Some(e) = precondition_can_succeed(skillset, precondition) {
        v.push(e);
    }
    if let Some(e) = precondition_can_fail(skillset, precondition) {
        v.push(e);
    }
    v
}

pub fn check_skill_preconditions(skillset: &Skillset, skill: &Skill) -> Vec<VError> {
    skill
        .preconditions()
        .iter()
        .map(|p| check_precondition(skillset, p))
        .flatten()
        .collect()
}
