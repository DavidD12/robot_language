use super::*;
use crate::model::*;

pub fn can_interrupt_effects_fail(skillset: &Skillset, skill: &Skill) -> Option<VError> {
    let interrupt = match skill.interrupt() {
        Some(i) => i,
        None => return None,
    };
    if interrupt.effects().is_empty() {
        return None;
    }
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(true);
    // Invariant OK
    for inv in skill.invariants().iter() {
        solver.assert(&smt.to_bool(inv.guard(), false));
    }
    // Apply effects
    solver.assert(&smt.apply_effects(interrupt.effects()));
    // Check Effects
    solver.assert(&z3::ast::Bool::not(&smt.check_effects(interrupt.effects())));
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => None,
        z3::SatResult::Unknown => None,
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let solution = smt.get_solution(&model, true);
            Some(VError::SkillInterruptEffectCanFail(skill.id(), solution))
        }
    }
}

pub fn can_success_effects_fail(skillset: &Skillset, success: &Success) -> Option<VError> {
    if success.effects().is_empty() {
        return None;
    }
    let SuccessId(skill_id, _) = success.id();
    let skill = skillset.get(skill_id).unwrap();
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(true);
    // Invariant OK
    for inv in skill.invariants().iter() {
        solver.assert(&smt.to_bool(inv.guard(), false));
    }
    // Apply effects
    solver.assert(&smt.apply_effects(success.effects()));
    // Check Effects
    solver.assert(&z3::ast::Bool::not(&smt.check_effects(success.effects())));
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => None,
        z3::SatResult::Unknown => None,
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let solution = smt.get_solution(&model, true);
            Some(VError::SkillSuccessEffectCanFail(success.id(), solution))
        }
    }
}

pub fn can_failure_effects_fail(skillset: &Skillset, failure: &Failure) -> Option<VError> {
    if failure.effects().is_empty() {
        return None;
    }
    let FailureId(skill_id, _) = failure.id();
    let skill = skillset.get(skill_id).unwrap();
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);
    let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
    // Resource
    smt.add_resources(true);
    // Invariant OK
    for inv in skill.invariants().iter() {
        solver.assert(&smt.to_bool(inv.guard(), false));
    }
    // Apply effects
    solver.assert(&smt.apply_effects(failure.effects()));
    // Check Effects
    solver.assert(&z3::ast::Bool::not(&smt.check_effects(failure.effects())));
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => None,
        z3::SatResult::Unknown => None,
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let solution = smt.get_solution(&model, true);
            Some(VError::SkillFailureEffectCanFail(failure.id(), solution))
        }
    }
}

pub fn check_skill_terminates(skillset: &Skillset, skill: &Skill) -> Vec<VError> {
    let mut v = vec![];
    if let Some(e) = can_interrupt_effects_fail(skillset, skill) {
        v.push(e);
    }
    v.extend(
        skill
            .successes()
            .iter()
            .map(|s| can_success_effects_fail(skillset, s))
            .flatten(),
    );
    v.extend(
        skill
            .failures()
            .iter()
            .map(|f| can_failure_effects_fail(skillset, f))
            .flatten(),
    );
    v
}
