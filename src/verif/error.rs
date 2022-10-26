use super::*;
use crate::model::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VError {
    // Event
    EventGuardCantSucceed(EventId),
    EventGuardCantFail(EventId),
    EventEffectCanFail(EventId, Solution),
    // Precondition
    SkillPreconditionCantSucceed(PreconditionId),
    SkillPreconditionCantFail(PreconditionId),
    // Start
    SkillStartEffectCanFail(SkillId, Solution),
    // Invariant
    SkillInvariantCantSucceed(InvariantId),
    SkillInvariantCantFail(InvariantId),
    SkillInvariantEffectCanFail(InvariantId, Solution),
    SkillStartInvariantCanFail(InvariantId, Solution),
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
            // Invariant
            VError::SkillInvariantCantSucceed(i) => {
                let InvariantId(skill_id, _) = i;
                let skill = model.get(*skill_id).unwrap();
                let inv = model.get(*i).unwrap();
                format!(
                    "skill '{}' invariant '{}' can't succeed.",
                    skill.name(),
                    inv.name()
                )
            }
            VError::SkillInvariantCantFail(i) => {
                let InvariantId(skill_id, _) = i;
                let skill = model.get(*skill_id).unwrap();
                let inv = model.get(*i).unwrap();
                format!(
                    "skill '{}' invariant '{}' can't fail.",
                    skill.name(),
                    inv.name()
                )
            }
            VError::SkillInvariantEffectCanFail(i, sol) => {
                let InvariantId(skill_id, _) = i;
                let skill = model.get(*skill_id).unwrap();
                let inv = model.get(*i).unwrap();
                format!(
                    "skill '{}' invariant '{}' effect can fail: {}",
                    skill.name(),
                    inv.name(),
                    sol.to_lang(model)
                )
            }
            VError::SkillStartInvariantCanFail(i, sol) => {
                let InvariantId(skill_id, _) = i;
                let skill = model.get(*skill_id).unwrap();
                let inv = model.get(*i).unwrap();
                format!(
                    "skill '{}' invariant '{}' start can fail at start: {}",
                    skill.name(),
                    inv.name(),
                    sol.to_lang(model)
                )
            }
        }
    }
}
