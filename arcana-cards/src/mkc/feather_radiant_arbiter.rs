//! Feather, Radiant Arbiter — `{R}{W}{W}` 4/3 Legendary Angel.
//! Flying, lifelink.
//! "Whenever you cast a noncreature spell that targets only Feather, you may
//! choose any number of other creatures that spell could target and pay {2}
//! for each of those creatures. If you do, for each of those creatures, copy
//! that spell. The copy targets that creature."
//!
//! The triggered copy ability is GAP'd: it requires "targets only this
//! creature" spell-cast detection, an arbitrary number of legal-target picks,
//! a per-creature {2} payment, and a retargeted copy per chosen creature —
//! none expressible with the demonstrated API.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feather, Radiant Arbiter");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: None,
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: feather_copy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn feather_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "noncreature spell that targets only Feather → choose any number of
    // other targetable creatures, pay {2} each, copy the spell once per chosen
    // creature retargeted to it" — no targets-only detection, per-creature
    // optional payment, or retargeted-copy primitive available.
    Vec::new()
}
