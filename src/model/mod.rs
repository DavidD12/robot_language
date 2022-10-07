pub mod reference;
pub use reference::*;

pub mod model;
pub use model::*;

pub mod rl_type;
pub use rl_type::*;

pub mod skillset;
pub use skillset::*;

pub mod data;
pub use data::*;

pub mod resource;
pub use resource::*;
pub mod state;
pub use state::*;
pub mod transition;
pub use transition::*;

pub mod event;
pub use event::*;

pub mod skill;
pub use skill::*;
pub mod precondition;
pub use precondition::*;

pub mod effect;
pub use effect::*;

pub mod expr;
pub use expr::*;

pub trait ToLang {
    fn to_lang(&self, model: &Model) -> String;
}
