use super::*;
use crate::model::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VError {
    // Event
    EventGuardCantSucceed(EventId),
    EventGuardCantFail(EventId),
    EventEffectCanFail(EventId, Solution),
    //
    SkillPreconditionCantSucceed(PreconditionId),
    SkillPreconditionCantFail(PreconditionId),
    //
    SkillStartEffectCanFail(SkillId, Solution),
    SkillStartCanFail(SkillId, Solution),
}

impl ToLang for VError {
    fn to_lang(&self, model: &Model) -> String {
        match self {
            // Event
            VError::EventGuardCantSucceed(e) => format!(
                "event '{}' guard can't succeed.",
                model.get(*e).unwrap().name()
            ),
            VError::EventGuardCantFail(e) => format!(
                "event '{}' guard can't fail.",
                model.get(*e).unwrap().name()
            ),

            VError::EventEffectCanFail(e, sol) => format!(
                "event '{}' effects can fail: {}",
                model.get(*e).unwrap().name(),
                sol.to_lang(model)
            ),
            // Precondition
            VError::SkillPreconditionCantSucceed(p) => format!(
                "precondition '{}' can't succeed.",
                model.get(*p).unwrap().name()
            ),
            VError::SkillPreconditionCantFail(p) => format!(
                "precondition '{}' can't fail.",
                model.get(*p).unwrap().name()
            ),
            // Start
            VError::SkillStartEffectCanFail(s, sol) => format!(
                "skill start effect '{}' can fail: {}",
                model.get(*s).unwrap().name(),
                sol.to_lang(model)
            ),
            VError::SkillStartCanFail(s, sol) => format!(
                "skill start '{}' can fail: {}",
                model.get(*s).unwrap().name(),
                sol.to_lang(model)
            ),
        }
    }
}
