use super::*;
use crate::model::*;

pub fn can_start_effects_fail(skillset: &Skillset, skill: &Skill) -> Option<VError> {
    if skill.start().is_empty() {
        return None;
    }
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
    // Apply effects
    solver.assert(&smt.apply_effects(skill.start()));
    // Check Effects
    solver.assert(&z3::ast::Bool::not(&smt.check_effects(skill.start())));
    // Solve
    match solver.check() {
        z3::SatResult::Unsat => None,
        z3::SatResult::Unknown => None,
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();
            let solution = smt.get_solution(&model, true);
            Some(VError::SkillStartEffectCanFail(skill.id(), solution))
        }
    }
}
