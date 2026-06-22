//! Cavalry Pegasus — `{1}{W}` 1/1 white Pegasus with Flying.
//!
//! Oracle text:
//! * Flying.
//! * Whenever this creature attacks, each attacking Human gains flying
//!   until end of turn.
//!
//! Implemented: the Flying keyword and the attack-trigger SHAPE is
//! wired.
//!
//! GAP: the trigger grants flying to "each attacking Human" — the
//! documented `ObjectFilter` refinements can restrict by subtype but not
//! to creatures that are currently ATTACKING, so the effect cannot be
//! scoped correctly (granting to all Humans would over-grant to
//! non-attackers). The resolver returns no effects.

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
    let name = reg.interner_mut().intern("Cavalry Pegasus");
    let pegasus = reg.interner_mut().intern("Pegasus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pegasus);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: each_attacking_human_gains_flying,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn each_attacking_human_gains_flying(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each attacking Human gains flying" — no documented
    // ObjectFilter refinement restricts a subtype set to creatures that
    // are currently attacking.
    Vec::new()
}
