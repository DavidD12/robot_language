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
    // Terminate
    SkillInterruptEffectCanFail(SkillId, Solution),
    SkillSuccessEffectCanFail(SuccessId, Solution),
    SkillFailureEffectCanFail(FailureId, Solution),
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
            // Terminate
            VError::SkillInterruptEffectCanFail(s, sol) => {
                let skill = model.get(*s).unwrap();
                format!(
                    "skill '{}' interrupt effect can fail: {}",
                    skill.name(),
                    sol.to_lang(model)
                )
            }
            VError::SkillSuccessEffectCanFail(s, sol) => {
                let SuccessId(skill_id, _) = s;
                let skill = model.get(*skill_id).unwrap();
                let success = model.get(*s).unwrap();
                format!(
                    "skill '{}' success '{}' effect can fail: {}",
                    skill.name(),
                    success.name(),
                    sol.to_lang(model)
                )
            }
            VError::SkillFailureEffectCanFail(f, sol) => {
                let FailureId(skill_id, _) = f;
                let skill = model.get(*skill_id).unwrap();
                let failure = model.get(*f).unwrap();
                format!(
                    "skill '{}' failure '{}' effect can fail: {}",
                    skill.name(),
                    failure.name(),
                    sol.to_lang(model)
                )
            }
        }
    }
}
