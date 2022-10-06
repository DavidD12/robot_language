use super::*;

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
