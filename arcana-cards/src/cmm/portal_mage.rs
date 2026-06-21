//! Portal Mage — `{2}{U}` 2/2 Human Wizard with Flash.
//! "Flash.
//!  When this creature enters during the declare attackers step, you may
//!  reselect which player or permanent target attacking creature is
//!  attacking."
//!
//! Flash is a base keyword. The ETB trigger is recorded as
//! `SelfEntersBattlefield`; its payload is GAP'd:
//! * GAP: the "during the declare attackers step" gate is not an intervening-
//!   if board predicate (no `conditions::` helper checks the current step),
//!   and the "reselect which player or permanent an attacking creature is
//!   attacking" payload (re-aiming a declared attacker) has no Effect
//!   primitive. The whole payload is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Portal Mage");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: reselect_attacker,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn reselect_attacker(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "during the declare attackers step" step-gate (no step-predicate
    // intervening-if) + "reselect which player/permanent a target attacking
    // creature is attacking" (no attacker-re-aim Effect primitive).
    Vec::new()
}
