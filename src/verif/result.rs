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
}
