//! Space Beleren — `{2}{W}{U}` Legendary Planeswalker — Jace,
//! starting loyalty 5.
//!
//! Space sculptor (divides the battlefield into alpha/beta/gamma sectors):
//! GAP — the sector subsystem is bespoke and not modeled. All three loyalty
//! abilities operate on sectors, so each is shelled with its correct loyalty
//! cost and a GAP body.
//!
//! +1: Creatures in each sector can be blocked this turn only by creatures in
//!     the same sector. GAP.
//! −1: Put a +1/+1 counter on each creature in the sector of your choice. GAP.
//! −5: Destroy all creatures in the sector of your choice. GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Space Beleren");
    let jace = reg.interner_mut().intern("Jace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Creatures in each sector can be blocked this turn only \
                       by creatures in the same sector.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_sector,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Put a +1/+1 counter on each creature in the sector of \
                       your choice.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_sector,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: Destroy all creatures in the sector of your choice.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_sector,
            }),
    )
}

fn gap_sector(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the Space-sculptor sector subsystem is not modeled.
    Vec::new()
}
