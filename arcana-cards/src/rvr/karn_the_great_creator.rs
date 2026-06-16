//! Karn, the Great Creator — `{4}` legendary planeswalker, starting
//! loyalty 5. Subtype Karn; colorless. (The static "opponents' artifact
//! activated abilities can't be activated" is a continuous ability, not a
//! loyalty ability — not modeled here.)
//!
//! Loyalty abilities:
//! * `+1`: Until your next turn, up to one target noncreature artifact
//!   becomes an artifact creature with power and toughness equal to its
//!   mana value. GAP — dynamic P/T (equal to mana value) isn't expressible.
//! * `−2`: Reveal an artifact card from outside the game (or a face-up
//!   exiled one you own) and put it into your hand. GAP — outside-the-game
//!   / sideboard / face-up-exile selection isn't expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karn, the Great Creator");
    let karn = reg.interner_mut().intern("Karn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(karn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, up to one target noncreature \
                       artifact becomes an artifact creature with power and \
                       toughness each equal to its mana value."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::ARTIFACT.into())
                            .without_types(TypeLine::CREATURE.into()),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: You may reveal an artifact card you own from \
                       outside the game or choose a face-up artifact card you \
                       own in exile. Put that card into your hand."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            }),
    )
}

fn plus_one_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: dynamic P/T equal to mana value (SetBasePT takes fixed values).
    Vec::new()
}

fn minus_two_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: outside-the-game / sideboard / face-up-exile artifact selection
    // is not expressible.
    Vec::new()
}
