use super::Solution;
use crate::model::*;

// use z3::ast::*;
// use z3::{Config, Context, Solver, Sort};
use z3::ast::Ast;

use std::collections::HashMap;

pub struct Smt<'a> {
    skillset: &'a Skillset,
    //
    _cfg: &'a z3::Config,
    ctx: &'a z3::Context,
    solver: &'a z3::Solver<'a>,
    //
    resource_sort: HashMap<ResourceId, z3::DatatypeSort<'a>>,
    resource_current: HashMap<ResourceId, z3::ast::Datatype<'a>>,
    resource_next: HashMap<ResourceId, z3::ast::Datatype<'a>>,
}

impl<'a> Smt<'a> {
    pub fn empty(
        skillset: &'a Skillset,
        cfg: &'a z3::Config,
        ctx: &'a z3::Context,
        solver: &'a z3::Solver,
    ) -> Self {
        Self {
            skillset,
            _cfg: cfg,
            ctx,
            solver,
            resource_sort: HashMap::new(),
            resource_current: HashMap::new(),
            resource_next: HashMap::new(),
        }
    }

    fn add_resource(&mut self, resource: &Resource) {
        let mut builder = z3::DatatypeBuilder::new(self.ctx, resource.name());
        for x in resource.states().iter() {
            builder = builder.variant(x.name(), Vec::new());
        }
        let datatype = builder.finish();
        self.resource_sort.insert(resource.id(), datatype);
    }

    fn add_resource_current(&mut self, resource: &Resource) {
        let datatype = &self.resource_sort[&resource.id()];
        let current = z3::ast::Datatype::new_const(
            self.ctx,
            format!("resource_{}_current", resource.name()),
            &datatype.sort,
        );
        self.resource_current.insert(resource.id(), current);
    }

    fn add_resource_next(&mut self, resource: &Resource) {
        let datatype = &self.resource_sort[&resource.id()];
        let current = z3::ast::Datatype::new_const(
            self.ctx,
            format!("resource_{}_next", resource.name()),
            &datatype.sort,
        );
        self.resource_next.insert(resource.id(), current);
    }

    pub fn add_resources(&mut self, next: bool) {
        for x in self.skillset.resources().iter() {
            self.add_resource(x);
            self.add_resource_current(x);
            if next {
                self.add_resource_next(x);
            }
        }
    }

    fn get_state(&self, id: StateId) -> z3::ast::Datatype {
        let StateId(resource_id, id) = id;
        let datatype = self.resource_sort.get(&resource_id).unwrap();
        datatype.variants[id]
            .constructor
            .apply(&[])
            .as_datatype()
            .unwrap()
    }

    fn exists_transition(
        &'a self,
        resource: ResourceId,
        src: z3::ast::Datatype<'a>,
        dst: z3::ast::Datatype<'a>,
    ) -> z3::ast::Bool {
        let resource = self.skillset.get(resource).unwrap();
        match resource.transitions() {
            Transitions::All => z3::ast::Bool::from_bool(self.ctx, true),
            Transitions::List(transitions) => {
                let mut v = vec![src._eq(&dst)];
                for tr in transitions {
                    let x = self.get_state(tr.src().resolved());
                    let y = self.get_state(tr.dst().resolved());
                    let a = vec![src._eq(&x), dst._eq(&y)];
                    let b = z3::ast::Bool::and(self.ctx, &a.iter().collect::<Vec<_>>());
                    v.push(b);
                }
                z3::ast::Bool::or(self.ctx, &v.iter().collect::<Vec<_>>())
            }
        }
    }

    pub fn to_bool(&self, expr: &Expr, next: bool) -> z3::ast::Bool {
        match expr {
            Expr::True => z3::ast::Bool::from_bool(self.ctx, true),
            Expr::False => z3::ast::Bool::from_bool(self.ctx, true),
            Expr::ResourceEq(r, s) => {
                let r_id = r.resolved();
                let s_id = s.resolved();
                let resource_state = if next {
                    &self.resource_next[&r_id]
                } else {
                    &self.resource_current[&r_id]
                };
                let state = self.get_state(s_id);
                resource_state._eq(&state)
            }
            Expr::ResourceNe(r, s) => {
                let r_id = r.resolved();
                let s_id = s.resolved();
                let resource_state = if next {
                    &self.resource_next[&r_id]
                } else {
                    &self.resource_current[&r_id]
                };
                let state = self.get_state(s_id);
                z3::ast::Bool::not(&resource_state._eq(&state))
            }
            Expr::Not(e) => z3::ast::Bool::not(&self.to_bool(e, next)),
            Expr::And(l, r) => {
                z3::ast::Bool::and(self.ctx, &[&self.to_bool(l, next), &self.to_bool(r, next)])
            }
            Expr::Or(l, r) => {
                z3::ast::Bool::or(self.ctx, &[&self.to_bool(l, next), &self.to_bool(r, next)])
            }
            Expr::Implies(l, r) => {
                z3::ast::Bool::implies(&self.to_bool(l, next), &self.to_bool(r, next))
            }
        }
    }

    pub fn check_effects(&self, effects: &Vec<Effect>) -> z3::ast::Bool {
        let v = effects
            .iter()
            .map(|x| self.check_effect(x))
            .collect::<Vec<_>>();
        z3::ast::Bool::and(self.ctx, &v.iter().collect::<Vec<_>>())
    }

    fn check_effect(&self, effect: &Effect) -> z3::ast::Bool {
        let resource = self.skillset.get(effect.resource().resolved()).unwrap();
        let r = self.resource_current[&resource.id()].clone();
        let s = self.get_state(effect.state().resolved());
        self.exists_transition(resource.id(), r, s)
    }

    pub fn apply_effects(&self, effects: &Vec<Effect>) -> z3::ast::Bool {
        let mut changed = Vec::new();
        let mut v = Vec::new();
        // Changed
        for e in effects.iter() {
            let resource_id = e.resource().resolved();
            let r = &self.resource_next[&resource_id];
            let s = self.get_state(e.state().resolved());
            v.push(r._eq(&s));
            changed.push(resource_id)
        }
        // Unchanged
        for resource in self.skillset.resources() {
            let resource_id = resource.id();
            if !changed.contains(&resource_id) {
                let current = &self.resource_current[&resource_id];
                let next = &self.resource_next[&resource_id];
                v.push(current._eq(&next))
            }
        }
        z3::ast::Bool::and(self.ctx, &v.iter().collect::<Vec<_>>())
    }

    pub fn get_solution(self, model: &z3::Model, next: bool) -> Solution {
        let mut solution = Solution::empty(self.skillset.id());
        for resource in self.skillset.resources().iter() {
            let state = model
                .eval(&self.resource_current[&resource.id()], true)
                .unwrap();
            let state = resource.get_state_from_name(&state.to_string()).unwrap();
            solution.current.insert(resource.id(), state.id());
        }
        if next {
            let mut map = HashMap::new();
            for resource in self.skillset.resources().iter() {
                let state = model
                    .eval(&self.resource_next[&resource.id()], true)
                    .unwrap();
                let state = resource.get_state_from_name(&state.to_string()).unwrap();
                map.insert(resource.id(), state.id());
            }
            solution.next = Some(map);
        }
        solution
    }
}
