use crate::model::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solution {
    pub id: SkillsetId,
    pub current: HashMap<ResourceId, StateId>,
    pub next: Option<HashMap<ResourceId, StateId>>,
}
impl Solution {
    pub fn empty(id: SkillsetId) -> Self {
        Self {
            id,
            current: HashMap::new(),
            next: None,
        }
    }

    pub fn pretty(&self, skillset: &Skillset) -> String {
        let mut s = String::from("initial: ");
        if !skillset.resources().is_empty() {
            let resources = skillset.resources();
            s.push_str(&format!(
                "{}=={}",
                resources[0],
                skillset.get(self.current[&resources[0].id()]).unwrap()
            ));
            for resource in &resources[1..] {
                s.push_str(&format!(
                    " and {}=={}",
                    resource,
                    skillset.get(self.current[&resource.id()]).unwrap()
                ));
            }
        }
        s.push_str("\n");
        if let Some(next) = &self.next {
            s.push_str("next: ");
            if !skillset.resources().is_empty() {
                let resources = skillset.resources();
                s.push_str(&format!(
                    "{}=={}",
                    resources[0],
                    skillset.get(next[&resources[0].id()]).unwrap()
                ));
                for resource in &resources[1..] {
                    s.push_str(&format!(
                        " and {}=={}",
                        resource,
                        skillset.get(next[&resource.id()]).unwrap()
                    ));
                }
            }
            s.push_str("\n");
        }
        s
    }
}

impl ToLang for Solution {
    fn to_lang(&self, model: &Model) -> String {
        let skillset = model.get(self.id).unwrap();
        let mut s = String::from("initial: {");
        if !skillset.resources().is_empty() {
            let resources = skillset.resources();
            s.push_str(&format!(
                "{}=={}",
                resources[0],
                skillset.get(self.current[&resources[0].id()]).unwrap()
            ));
            for resource in &resources[1..] {
                s.push_str(&format!(
                    " and {}=={}",
                    resource,
                    skillset.get(self.current[&resource.id()]).unwrap()
                ));
            }
        }
        s.push_str("}");
        if let Some(next) = &self.next {
            s.push_str(" next {");
            if !skillset.resources().is_empty() {
                let resources = skillset.resources();
                s.push_str(&format!(
                    "{}=={}",
                    resources[0],
                    skillset.get(next[&resources[0].id()]).unwrap()
                ));
                for resource in &resources[1..] {
                    s.push_str(&format!(
                        " and {}=={}",
                        resource,
                        skillset.get(next[&resource.id()]).unwrap()
                    ));
                }
            }
            s.push_str("}   ");
        }
        s
    }
}
