use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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
    events: Vec<Event>,
    skills: Vec<Skill>,
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
            events: Vec::new(),
            skills: Vec::new(),
            position,
        }
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

    pub fn find_resource(&self, name: &str) -> Option<ResourceId> {
        for x in self.resources.iter() {
            if x.name() == name {
                return Some(x.id());
            }
        }
        None
    }

    pub fn resource_map(&self) -> HashMap<String, ResourceId> {
        let mut map = HashMap::new();
        for x in self.resources.iter() {
            map.insert(x.name().into(), x.id());
        }
        map
    }

    //---------- State ----------

    pub fn get_state(&self, id: StateId) -> Option<&State> {
        let StateId(resource_id, _) = id;
        let resource = self.get_resource(resource_id)?;
        resource.get_state(id)
    }

    pub fn state_map(&self) -> HashMap<String, StateId> {
        let mut map = HashMap::new();
        for x in self.resources.iter() {
            for y in x.states().iter() {
                map.insert(y.name().into(), y.id());
            }
        }
        map
    }

    //---------- Event ----------

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn get_event(&self, id: EventId) -> Option<&Event> {
        let EventId(skillset_id, event_id) = id;
        if self.id != skillset_id {
            None
        } else {
            self.events.get(event_id)
        }
    }

    pub fn add_event(&mut self, mut event: Event) -> EventId {
        let id = EventId(self.id, self.events.len());
        event.set_id(id);
        self.events.push(event);
        id
    }

    //---------- Skill ----------

    pub fn skills(&self) -> &Vec<Skill> {
        &self.skills
    }

    pub fn get_skill(&self, id: SkillId) -> Option<&Skill> {
        let SkillId(skillset_id, event_id) = id;
        if self.id != skillset_id {
            None
        } else {
            self.skills.get(event_id)
        }
    }

    pub fn add_skill(&mut self, mut skill: Skill) -> SkillId {
        let id = SkillId(self.id, self.skills.len());
        skill.set_id(id);
        self.skills.push(skill);
        id
    }

    //---------- Duplicate ----------

    pub fn names(&self) -> Vec<Naming> {
        let mut v = Vec::new();
        // Data
        for x in self.data.iter() {
            v.push((x.name().into(), x.position()));
        }
        // Resource
        for x in self.resources.iter() {
            v.push((x.name().into(), x.position()));
            v.extend(x.names());
        }
        // Event
        for x in self.events.iter() {
            v.push((x.name().into(), x.position()));
        }
        // Skill
        for x in self.skills.iter() {
            v.push((x.name().into(), x.position()));
        }
        v
    }

    pub fn duplicate(&self, container_names: &Vec<Naming>) -> Result<(), RlError> {
        let mut global_names: Vec<Naming> = container_names
            .iter()
            .cloned()
            .chain(self.names().iter().cloned())
            .collect();
        global_names.extend(self.names());
        check_duplicate(&global_names)?;

        // Skill
        for x in self.skills.iter() {
            // x.duplicate(&global_names)?;
        }
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        // Data
        for x in self.data.iter_mut() {
            x.resolve_type(map)?;
        }
        // Skill
        for x in self.skills.iter_mut() {
            x.resolve_type(&map)?;
        }
        Ok(())
    }

    pub fn resolve_resource(&mut self) -> Result<(), RlError> {
        let map = self.resource_map();
        // Event
        for x in self.events.iter_mut() {
            x.resolve_resource(&map)?;
        }
        // Skill
        for x in self.skills.iter_mut() {
            x.resolve_resource(&map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self) -> Result<(), RlError> {
        // Resource
        for x in self.resources.iter_mut() {
            x.resolve_state()?;
        }
        // Event
        let map = self.state_map();
        for x in self.events.iter_mut() {
            x.resolve_state(&map)?;
        }
        // Skill
        let map = self.state_map();
        for x in self.skills.iter_mut() {
            x.resolve_state(&map)?;
        }
        Ok(())
    }
}

impl Named<SkillsetId> for Skillset {
    fn id(&self) -> SkillsetId {
        self.id
    }
    fn set_id(&mut self, id: SkillsetId) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
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
                s.push_str(&format!("\t\t{}", &x.to_lang(model)));
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
        // Event
        if !self.events.is_empty() {
            s.push_str("\tevent {\n");
            for x in self.events.iter() {
                s.push_str(&x.to_lang(model));
            }
            s.push_str("\t}\n");
        }
        // Skill
        if !self.skills.is_empty() {
            s.push_str("\tskill {\n");
            for x in self.skills.iter() {
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
