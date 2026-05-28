//! Soldevi Adnate — `{1}{B}` 1/2 Creature — Human Cleric.
//! `{T}, Sacrifice a black or artifact creature: Add an amount of {B} equal to the sacrificed
//!  creature's mana value.`
//! GAP: "sacrifice a black or artifact creature" — ActivationCost.sacrifice = true sacrifices SELF.
//! GAP: Amount = sacrificed creature's mana value — not trackable after sacrifice.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soldevi Adnate");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a black or artifact creature: Add an amount of {B} equal to the sacrificed creature's mana value.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: "sacrifice another black or artifact creature" — no such ActivationCost field
                // GAP: dynamic {B} = sacrificed creature's mana value — not trackable
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black,
            }),
    )
}

fn add_black(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: dynamic amount = sacrificed creature's mana value — not accessible
    Vec::new()
}
