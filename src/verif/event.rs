use super::*;
use crate::model::*;

pub fn event_guard_can_succeed(skillset: &Skillset, event: &Event) -> Option<VError> {
    match event.guard() {
        None => None,
        Some(guard) => {
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);
            let solver = z3::Solver::new(&ctx);
            let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
            // Resource
            smt.add_resources(false);
            // Event guard
            solver.assert(&smt.to_bool(guard, false));
            // Solve
            match solver.check() {
                z3::SatResult::Unsat => Some(VError::EventGuardCantSucceed(event.id())),
                z3::SatResult::Unknown => panic!("unknown"),
                z3::SatResult::Sat => None,
            }
        }
    }
}

pub fn event_guard_can_fail(skillset: &Skillset, event: &Event) -> Option<VError> {
    match event.guard() {
        None => None,
        Some(guard) => {
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);
            let solver = z3::Solver::new(&ctx);
            let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
            // Resource
            smt.add_resources(false);
            // Event guard
            solver.assert(&z3::ast::Bool::not(&smt.to_bool(guard, false)));
            // Solve
            match solver.check() {
                z3::SatResult::Unsat => Some(VError::EventGuardCantFail(event.id())),
                z3::SatResult::Unknown => panic!("unknown"),
                z3::SatResult::Sat => None,
            }
        }
    }
}

pub fn can_event_effects_fail(skillset: &Skillset, event: &Event) -> Option<VError> {
    match event.guard() {
        None => None,
        Some(guard) => {
            if event.effects().is_empty() {
                return None;
            }
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);
            let solver = z3::Solver::new(&ctx);
            let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
            // Resource
            smt.add_resources(true);
            // Event guard
            solver.assert(&smt.to_bool(guard, false));
            // Apply effects
            solver.assert(&smt.apply_effects(event.effects()));
            // Check Effects
            solver.assert(&z3::ast::Bool::not(&smt.check_effects(event.effects())));
            // Solve
            match solver.check() {
                z3::SatResult::Unsat => None,
                z3::SatResult::Unknown => None,
                z3::SatResult::Sat => {
                    let model = solver.get_model().unwrap();
                    let solution = smt.get_solution(&model, true);
                    Some(VError::EventEffectCanFail(event.id(), solution))
                }
            }
        }
    }
}

pub fn check_event(skillset: &Skillset, event: &Event) -> Vec<VError> {
    let mut v = vec![];
    if let Some(e) = event_guard_can_succeed(skillset, event) {
        v.push(e);
    }
    if let Some(e) = event_guard_can_fail(skillset, event) {
        v.push(e);
    }
    if let Some(e) = can_event_effects_fail(skillset, event) {
        v.push(e);
    }
    v
}

pub fn check_events(skillset: &Skillset) -> Vec<VError> {
    skillset
        .events()
        .iter()
        .map(|e| check_event(skillset, e))
        .flatten()
        .collect()
}
