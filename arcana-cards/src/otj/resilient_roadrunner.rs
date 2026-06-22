//! Resilient Roadrunner — `{1}{R}` 2/2 Bird.
//! "Haste, protection from Coyotes
//!  {3}: This creature can't be blocked this turn except by creatures with haste."
//!
//! Haste is an expressible keyword. Protection from Coyotes is NOT in the
//! usable KeywordAbility surface (parametrized Protection is not modeled) — GAP.
//! The activated ability is modeled as a plain "can't be blocked this turn";
//! the "except by creatures with haste" rider has no API surface — partial.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Resilient Roadrunner");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "protection from Coyotes" — parametrized Protection is not in the
        // usable KeywordAbility surface.
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}: This creature can't be blocked this turn except by creatures with haste."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: cant_be_blocked,
        }),
    )
}

fn cant_be_blocked(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "except by creatures with haste" — the blockable-exception rider has
    // no API surface; modeled as plain can't-be-blocked this turn.
    vec![Effect::CantBeBlocked {
        target: ctx.source,
        duration: Duration::EndOfTurn,
    }]
}
