use super::*;
use crate::{model::*, Solution};

pub fn can_event_succed(skillset: &Skillset, event: &Event) -> Option<Solution> {
    match event.guard() {
        None => Some(Solution::empty()),
        Some(guard) => {
            let cfg = z3::Config::new();
            let ctx = z3::Context::new(&cfg);
            let solver = z3::Solver::new(&ctx);
            let mut smt = Smt::empty(skillset, &cfg, &ctx, &solver);
            // Resource
            smt.add_resources(false);
            // Event guard
            solver.assert(&smt.expr_to_smt(guard, false));
            // Solve
            match solver.check() {
                z3::SatResult::Unsat => None,
                z3::SatResult::Unknown => None,
                z3::SatResult::Sat => {
                    let model = solver.get_model().unwrap();
                    let solution = smt.get_solution(&model, false);
                    Some(solution)
                }
            }
        }
    }
}
