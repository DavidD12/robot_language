use std::collections::HashMap;

use super::*;
use crate::parser::RlError;

pub struct Model {
    types: Vec<RlType>,
    skillsets: Vec<Skillset>,
}

impl Model {
    pub fn empty() -> Self {
        Self {
            types: Vec::new(),
            skillsets: Vec::new(),
        }
    }

    //---------- Type ----------

    pub fn types(&self) -> &Vec<RlType> {
        &self.types
    }

    pub fn add_type(&mut self, mut rl_type: RlType) -> TypeId {
        let id = TypeId(self.types.len());
        rl_type.set_id(id);
        self.types.push(rl_type);
        id
    }

    pub fn get_type(&self, id: TypeId) -> Option<&RlType> {
        let TypeId(index) = id;
        self.types.get(index)
    }

    pub fn find_type(&self, name: &str) -> Option<TypeId> {
        for x in self.types.iter() {
            if x.name() == name {
                return Some(x.id());
            }
        }
        None
    }

    pub fn type_map(&self) -> HashMap<String, TypeId> {
        let mut map = HashMap::new();
        for x in self.types.iter() {
            map.insert(x.name().into(), x.id());
        }
        map
    }

    //---------- Skillset ----------

    pub fn skillsets(&self) -> &Vec<Skillset> {
        &self.skillsets
    }

    pub fn add_skillset(&mut self, mut skillset: Skillset) -> SkillsetId {
        let id = SkillsetId(self.skillsets.len());
        skillset.set_id(id);
        self.skillsets.push(skillset);
        id
    }

    pub fn get_skillset(&self, id: SkillsetId) -> Option<&Skillset> {
        let SkillsetId(index) = id;
        self.skillsets.get(index)
    }

    pub fn find_skillset(&self, name: &str) -> Option<SkillsetId> {
        for x in self.skillsets.iter() {
            if x.name() == name {
                return Some(x.id());
            }
        }
        None
    }

    //---------- Data ----------

    pub fn get_data(&self, id: DataId) -> Option<&Data> {
        let DataId(skillset_id, _) = id;
        let skillset = self.get_skillset(skillset_id)?;
        skillset.get_data(id)
    }

    //---------- Resource ----------

    pub fn get_resource(&self, id: ResourceId) -> Option<&Resource> {
        let ResourceId(skillset_id, _) = id;
        let skillset = self.get_skillset(skillset_id)?;
        skillset.get_resource(id)
    }

    pub fn get_state(&self, id: StateId) -> Option<&State> {
        let StateId(resource_id, _) = id;
        let resource = self.get_resource(resource_id)?;
        resource.get_state(id)
    }

    //---------- Duplicate ----------

    pub fn duplicate(&self) -> Result<(), RlError> {
        self.duplicate_type()?;
        for x in self.skillsets.iter() {
            x.duplicate()?;
        }
        Ok(())
    }

    fn duplicate_type(&self) -> Result<(), RlError> {
        for (i, x) in self.types.iter().enumerate() {
            for y in self.types.iter().skip(i + 1) {
                if x.name() == y.name() {
                    return Err(RlError::Duplicate {
                        name: x.name().into(),
                        first: x.position(),
                        second: y.position(),
                    });
                }
            }
        }
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve(&mut self) -> Result<(), RlError> {
        self.resolve_type()
    }

    fn resolve_type(&mut self) -> Result<(), RlError> {
        let map = self.type_map();
        for x in self.skillsets.iter_mut() {
            x.resolve_type(&map)?;
        }
        Ok(())
    }

    //----------  ----------

    pub fn to_lang(&self) -> String {
        let mut s = String::new();
        // ----- Type -----
        match self.types.len() {
            0 => {}
            1 => s.push_str(&format!("type {}\n", self.types().get(0).unwrap())),
            _ => {
                s.push_str(&format!("type {{\n"));
                for t in self.types() {
                    s.push_str(&format!("\t{}\n", t));
                }
                s.push_str("}\n");
            }
        }
        // ----- Skillset -----
        for skillset in self.skillsets.iter() {
            s.push_str(&skillset.to_lang(self));
        }
        //
        s
    }
}

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_lang())
    }
}
