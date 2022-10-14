use super::*;
use crate::parser::Position;

#[derive(Clone)]
pub enum Reference<I: Id> {
    Unresolved(String, Option<Position>),
    Resolved(I),
}

impl<I: Id> Reference<I> {
    pub fn resolved(&self) -> I {
        match self {
            Reference::Unresolved(_, _) => panic!("reference must be resolved"),
            Reference::Resolved(id) => *id,
        }
    }
}

impl ToLang for Reference<TypeId> {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get(*id).unwrap()),
        }
    }
}
impl ToLang for Reference<ResourceId> {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get(*id).unwrap()),
        }
    }
}

impl ToLang for Reference<StateId> {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", model.get(*id).unwrap()),
        }
    }
}

// impl ToLang for Reference<DataId> {
//     fn to_lang(&self, model: &crate::Model) -> String {
//         match self {
//             Reference::Unresolved(name, _) => format!("{}/* ? */", name),
//             Reference::Resolved(id) => format!("{}", model.get(*id).unwrap()),
//         }
//     }
// }

// impl ToLang for Reference<SkillsetId> {
//     fn to_lang(&self, model: &crate::Model) -> String {
//         match self {
//             Reference::Unresolved(name, _) => format!("{}/* ? */", name),
//             Reference::Resolved(id) => format!("{}", model.get(*id).unwrap()),
//         }
//     }
// }

// impl ToLang for Reference<EventId> {
//     fn to_lang(&self, model: &Model) -> String {
//         match self {
//             Reference::Unresolved(name, _) => format!("{}/* ? */", name),
//             Reference::Resolved(id) => format!("{}", model.get(*id).unwrap()),
//         }
//     }
// }
