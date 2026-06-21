//! Plague Nurse — `{3}{G}` 3/4 Phyrexian Cleric.
//! Toxic 2.
//! {2}{G}: Each other creature you control with toxic gains toxic 1 until
//! end of turn. Activate only once each turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plague Nurse");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Toxic(2)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{G}: Each other creature you control with toxic gains toxic 1 until end of turn. Activate only once each turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                once_per_turn: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: grant_toxic,
        }),
    )
}

fn grant_toxic(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Each other creature you control with toxic gains toxic 1" — no
    // ObjectFilter predicate matches a parametrized Toxic(N) of any value
    // ("with toxic"), and granting Toxic per-id to a value-filtered board
    // set (plus the "other" self-exclusion) is not faithfully expressible.
    Vec::new()
}
