//! Erstwhile Trooper — `{1}{B}{G}` 2/2 Zombie Soldier.
//! `Discard a creature card: This creature gets +2/+2 and gains trample until end of turn. Activate only once each turn.`
//! "Discard a creature card" cost modeled via `discard_other` (creature-card
//! filter). "Activate only once each turn" enforced via `once_per_turn`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Erstwhile Trooper");
    let zombie = reg.interner_mut().intern("Zombie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a creature card: This creature gets +2/+2 and gains trample until end of turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    discard_other: Some(arcana_core::targets::ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..Default::default()
                    }),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_with_trample,
            }),
    )
}

fn pump_with_trample(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ActivationCost has no "discard a creature card" field.
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::Trample],
    }]
}
