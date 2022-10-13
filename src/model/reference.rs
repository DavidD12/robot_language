use super::*;
use crate::{Position, ToLang};

#[derive(Clone)]
pub enum Reference<I: Id> {
    Unresolved(String, Option<Position>),
    Resolved(I),
}

// impl<I: Id> ToLang for Reference<I> {
//     fn to_lang(&self, model: &crate::Model) -> String {
//         match self {
//             Reference::Unresolved(name, _) => format!("{}/* ? */", name),
//             Reference::Resolved(id) => format!("{}", model.get(*id).unwrap()),
//         }
//     }
// }

impl ToLang for Reference<TypeId> {
    fn to_lang(&self, model: &crate::Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get_type(*id).unwrap()),
        }
    }
}

impl ToLang for Reference<DataId> {
    fn to_lang(&self, model: &crate::Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get_data(*id).unwrap()),
        }
    }
}

impl ToLang for Reference<SkillsetId> {
    fn to_lang(&self, model: &crate::Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get_skillset(*id).unwrap()),
        }
    }
}

impl ToLang for Reference<ResourceId> {
    fn to_lang(&self, model: &crate::Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get_resource(*id).unwrap()),
        }
    }
}

impl ToLang for Reference<StateId> {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get_state(*id).unwrap()),
        }
    }
}

impl ToLang for Reference<EventId> {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get_event(*id).unwrap()),
        }
    }
}
