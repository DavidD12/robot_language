use super::*;
use crate::parser::Position;

#[derive(Clone, Copy, PartialEq)]
pub struct SkillsetId(pub usize);
impl Id for SkillsetId {
    fn empty() -> Self {
        Self(0)
    }
}

pub struct Skillset {
    id: SkillsetId,
    name: String,
    data: Vec<Data>,
    resources: Vec<Resource>,
    position: Option<Position>,
}

impl Skillset {
    pub fn empty<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = SkillsetId::empty();
        let name = name.into();
        Self {
            id,
            name,
            data: Vec::new(),
            resources: Vec::new(),
            position,
        }
    }

    pub fn id(&self) -> SkillsetId {
        self.id
    }

    pub(super) fn set_id(&mut self, id: SkillsetId) {
        self.id = id;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    //---------- Data ----------

    pub fn data(&self) -> &Vec<Data> {
        &self.data
    }

    pub fn get_data(&self, id: DataId) -> Option<&Data> {
        let DataId(skillset_id, data_id) = id;
        if self.id != skillset_id {
            None
        } else {
            self.data.get(data_id)
        }
    }

    pub fn add_data(&mut self, mut data: Data) -> DataId {
        let id = DataId(self.id, self.data.len());
        data.set_id(id);
        self.data.push(data);
        id
    }

    //---------- Resource ----------

    pub fn resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    pub fn get_resource(&self, id: ResourceId) -> Option<&Resource> {
        let ResourceId(skillset_id, resource_id) = id;
        if self.id != skillset_id {
            None
        } else {
            self.resources.get(resource_id)
        }
    }

    pub fn add_resource(&mut self, mut resource: Resource) -> ResourceId {
        let id = ResourceId(self.id, self.resources.len());
        resource.set_id(id);
        self.resources.push(resource);
        id
    }

    pub fn get_state(&self, id: StateId) -> Option<&State> {
        let StateId(resource_id, _) = id;
        let resource = self.get_resource(resource_id)?;
        resource.get_state(id)
    }

    //---------- ----------

    pub fn position(&self) -> Option<Position> {
        self.position
    }
}

impl ToLang for Skillset {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = String::new();
        s.push_str(&format!("skillset {} {{\n", self.name));
        // Data
        if !self.data.is_empty() {
            s.push_str("\tdata {\n");
            for x in self.data.iter() {
                s.push_str(&x.to_lang(model));
            }
            s.push_str("\t}\n");
        }
        // Resource
        if !self.resources.is_empty() {
            s.push_str("\tresource {\n");
            for x in self.resources.iter() {
                s.push_str(&x.to_lang(model));
            }
            s.push_str("\t}\n");
        }
        //
        s.push_str("}\n");
        s
    }
}

impl std::fmt::Display for Skillset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
