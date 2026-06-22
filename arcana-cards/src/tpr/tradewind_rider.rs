//! Tradewind Rider — `{3}{U}` 1/4 blue Spirit.
//! Flying.
//! {T}, Tap two untapped creatures you control: Return target permanent to its
//! owner's hand.
//!
//! Abilities:
//!  - Flying → `KeywordAbility::Flying`.
//!  - Activated {T} + tap two untapped creatures you control: bounce target
//!    permanent. The tap-two-other-creatures cost maps to `tap_other` with
//!    `tap_other_count: 2`; the engine enumerates and taps them as the cost.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tradewind Rider");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Tap two untapped creatures you control: Return target \
                   permanent to its owner's hand."
                .into(),
            cost: ActivationCost {
                tap: true,
                tap_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                tap_other_count: 2,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(ObjectFilter::permanent()),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: bounce_target_permanent,
        }),
    )
}

fn bounce_target_permanent(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnToHand { target: *id }]
}
