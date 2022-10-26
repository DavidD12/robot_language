use super::*;
use crate::model::*;

pub fn invariant_can_succeed(skillset: &Skillset, invariant: &Invariant) -> Option<VError> {
    let InvariantId(skill_id, _) = invariant.id();
    let skill = skillset.get(skill_id).unwrap();
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(false);
    // Add previous invariants and current
    for inv in skill.invariants().iter() {
        solver.assert(&smt.to_bool(inv.guard(), false));
        if inv.id() == invariant.id() {
            break;
        }
    }
    match solver.check() {
        z3::SatResult::Unsat => Some(VError::SkillInvariantCantSucceed(invariant.id())),
        z3::SatResult::Unknown => panic!("unknown"),
        z3::SatResult::Sat => None,
    }
}

pub fn invariant_can_fail(skillset: &Skillset, invariant: &Invariant) -> Option<VError> {
    let InvariantId(skill_id, _) = invariant.id();
    let skill = skillset.get(skill_id).unwrap();
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(false);
    // Add previous invariants
    for inv in skill.invariants().iter() {
        if inv.id() == invariant.id() {
            break;
        }
        solver.assert(&smt.to_bool(inv.guard(), false));
    }
    // current fails
    solver.assert(&z3::ast::Bool::not(&smt.to_bool(invariant.guard(), false)));
    match solver.check() {
        z3::SatResult::Unsat => Some(VError::SkillInvariantCantFail(invariant.id())),
        z3::SatResult::Unknown => panic!("unknown"),
        z3::SatResult::Sat => None,
    }
}

pub fn can_invariant_effects_fail(skillset: &Skillset, invariant: &Invariant) -> Option<VError> {
    if invariant.effects().is_empty() {
        return None;
    }
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(true);
    // Invariant failure
    solver.assert(&z3::ast::Bool::not(&smt.to_bool(invariant.guard(), false)));
    // Apply effects
    solver.assert(&smt.apply_effects(invariant.effects()));
    // Check Effects
    solver.assert(&z3::ast::Bool::not(&smt.check_effects(invariant.effects())));
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => None,
        z3::SatResult::Unknown => None,
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let solution = smt.get_solution(&model, true);
            Some(VError::SkillInvariantEffectCanFail(
                invariant.id(),
                solution,
            ))
        }
    }
}

pub fn can_start_invariant_fail(skillset: &Skillset, invariant: &Invariant) -> Option<VError> {
    let InvariantId(skill_id, _) = invariant.id();
    let skill = skillset.get(skill_id).unwrap();
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(true);
    // Precondition
    for pre in skill.preconditions().iter() {
        solver.assert(&smt.to_bool(pre.expr(), false));
    }
    // Start Effects
    solver.assert(&smt.apply_effects(skill.start()));
    solver.assert(&smt.check_effects(skill.start()));
    // Previous Invariants
    for inv in skill.invariants().iter() {
        if inv.id() == invariant.id() {
            break;
        }
        solver.assert(&smt.to_bool(inv.guard(), true));
    }
    // Invariant failure
    solver.assert(&z3::ast::Bool::not(&smt.to_bool(invariant.guard(), true)));
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => None,
        z3::SatResult::Unknown => None,
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let solution = smt.get_solution(&model, true);
            Some(VError::SkillStartInvariantCanFail(invariant.id(), solution))
        }
    }
}

pub fn check_invariant(skillset: &Skillset, invariant: &Invariant) -> Vec<VError> {
    let mut v = vec![];
    if let Some(e) = invariant_can_succeed(skillset, invariant) {
        v.push(e)
    }
    if let Some(e) = invariant_can_fail(skillset, invariant) {
        v.push(e)
    }
    if let Some(e) = can_invariant_effects_fail(skillset, invariant) {
        v.push(e)
    }
    if let Some(e) = can_start_invariant_fail(skillset, invariant) {
        v.push(e)
    }
    v
}

pub fn check_skill_invariants(skillset: &Skillset, skill: &Skill) -> Vec<VError> {
    skill
        .invariants()
        .iter()
        .map(|i| check_invariant(skillset, i))
        .flatten()
        .collect()
}
