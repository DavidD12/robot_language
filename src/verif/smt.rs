use z3::ast::Datatype;

use crate::model::*;

// use z3::ast::*;
// use z3::{Config, Context, Solver, Sort};

use std::collections::HashMap;

pub struct Smt<'a> {
    model: &'a Model,
    //
    _cfg: &'a z3::Config,
    ctx: &'a z3::Context,
    solver: &'a z3::Solver<'a>,
    //
    resource_sort: HashMap<ResourceId, z3::DatatypeSort<'a>>,
}

impl<'a> Smt<'a> {
    pub fn empty(
        model: &'a Model,
        cfg: &'a z3::Config,
        ctx: &'a z3::Context,
        solver: &'a z3::Solver,
    ) -> Self {
        Self {
            model,
            _cfg: cfg,
            ctx,
            solver,
            resource_sort: HashMap::new(),
        }
    }

    fn add_resource(&mut self, resource: &Resource) {
        let mut builder = z3::DatatypeBuilder::new(self.ctx, resource.name());
        for x in resource.states().iter() {
            builder = builder.variant(x.name(), Vec::new());
        }
        let sort = builder.finish();
        self.resource_sort.insert(resource.id(), sort);
    }

    fn add_resources(&mut self, skillset: &Skillset) {
        for x in skillset.resources().iter() {
            self.add_resource(x);
        }
    }

    fn get_state(&self, state: &State) -> Datatype {
        let StateId(resource_id, id) = state.id();
        let sort = self.resource_sort.get(&resource_id).unwrap();
        sort.variants[id]
            .constructor
            .apply(&[])
            .as_datatype()
            .unwrap()
    }

    fn exists_transition(&self, x: &State, y: &State) -> z3::ast::Bool {
        // if x.id() == y.id() {
        //     return z3::ast::Bool::from_bool(self.ctx, true);
        // }
        // let StateId(x_res_id, _) = x.id();
        // let StateId(y_res_id, _) = y.id();
        // if x_res_id != y_res_id {
        //     return z3::ast::Bool::from_bool(self.ctx, false);
        // }
        // let resource = self.model.get_resource(x_res_id).unwrap();
        // match resource.transitions() {
        //     Transitions::All => z3::ast::Bool::from_bool(self.ctx, true),
        //     Transitions::List(l) => {
        //         for transition in l.iter() {
        //             if x == transition.src() && y == transition.dst() {
        //                 return z3::ast::Bool::from_bool(self.ctx, true);
        //             }
        //         }
        //     }
        // }
        z3::ast::Bool::from_bool(self.ctx, false)
    }
}
