use super::*;
use crate::model::*;

pub fn can_interrupt_postcondiion_fail(skillset: &Skillset, skill: &Skill) -> Option<VError> {
    let interrupt = match skill.interrupt() {
        Some(i) => i,
        None => return None,
    };
    match interrupt.postcondition() {
        None => None,
        Some(postcondition) => {
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);
            let solver = z3::Solver::new(&ctx);
            let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
            // Resource
            smt.add_resources(false);
            // Not Invariant
            for inv in skill.invariants().iter() {
                solver.assert(&z3::ast::Bool::not(&smt.to_bool(inv.guard(), false)));
            }
            // Postcondition
            solver.assert(&smt.to_bool(postcondition, false));
            // Solve
            match solver.check() {
                z3::SatResult::Unsat => None,
                z3::SatResult::Unknown => None,
                z3::SatResult::Sat => {
                    let model = solver.get_model().unwrap();
                    let solution = smt.get_solution(&model, false);
                    Some(VError::SkillInterruptPostconditionCanFail(
                        skill.id(),
                        solution,
                    ))
                }
            }
        }
    }
}

pub fn can_success_postcondiion_fail(skillset: &Skillset, success: &Success) -> Option<VError> {
    match success.postcondition() {
        None => None,
        Some(postcondition) => {
            let SuccessId(skill_id, _) = success.id();
            let skill = skillset.get(skill_id).unwrap();
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);
            let solver = z3::Solver::new(&ctx);
            let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
            // Resource
            smt.add_resources(false);
            // Not Invariant
            for inv in skill.invariants().iter() {
                solver.assert(&z3::ast::Bool::not(&smt.to_bool(inv.guard(), false)));
            }
            // Postcondition
            solver.assert(&smt.to_bool(postcondition, false));
            // Solve
            match solver.check() {
                z3::SatResult::Unsat => None,
                z3::SatResult::Unknown => None,
                z3::SatResult::Sat => {
                    let model = solver.get_model().unwrap();
                    let solution = smt.get_solution(&model, false);
                    Some(VError::SkillSuccessPostconditionCanFail(
                        success.id(),
                        solution,
                    ))
                }
            }
        }
    }
}

pub fn can_failure_postcondiion_fail(skillset: &Skillset, failure: &Failure) -> Option<VError> {
    match failure.postcondition() {
        None => None,
        Some(postcondition) => {
            let FailureId(skill_id, _) = failure.id();
            let skill = skillset.get(skill_id).unwrap();
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);
            let solver = z3::Solver::new(&ctx);
            let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
            // Resource
            smt.add_resources(false);
            // Not Invariant
            for inv in skill.invariants().iter() {
                solver.assert(&z3::ast::Bool::not(&smt.to_bool(inv.guard(), false)));
            }
            // Postcondition
            solver.assert(&smt.to_bool(postcondition, false));
            // Solve
            match solver.check() {
                z3::SatResult::Unsat => None,
                z3::SatResult::Unknown => None,
                z3::SatResult::Sat => {
                    let model = solver.get_model().unwrap();
                    let solution = smt.get_solution(&model, false);
                    Some(VError::SkillFailurePostconditionCanFail(
                        failure.id(),
                        solution,
                    ))
                }
            }
        }
    }
}

pub fn check_skill_postconditions(skillset: &Skillset, skill: &Skill) -> Vec<VError> {
    let mut v = vec![];
    if let Some(e) = can_interrupt_postcondiion_fail(skillset, skill) {
        v.push(e);
    }
    v.extend(
        skill
            .successes()
            .iter()
            .map(|s| can_success_postcondiion_fail(skillset, s))
            .flatten(),
    );
    v.extend(
        skill
            .failures()
            .iter()
            .map(|f| can_failure_postcondiion_fail(skillset, f))
            .flatten(),
    );
    v
}
