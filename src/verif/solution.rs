use crate::model::*;
use std::collections::HashMap;

pub struct Solution {
    pub current: HashMap<ResourceId, StateId>,
    pub next: Option<HashMap<ResourceId, StateId>>,
}
impl Solution {
    pub fn empty() -> Self {
        Self {
            current: HashMap::new(),
            next: None,
        }
    }
}
