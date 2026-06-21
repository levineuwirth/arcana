//! Thrull Parasite — `{B}` 1/1 Thrull.
//! Extort (keyword unsupported for this card class — GAP).
//! "{T}, Pay 2 life: Remove a counter from target nonland permanent."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thrull Parasite");
    let thrull = reg.interner_mut().intern("Thrull");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thrull);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: keyword — Extort (not in the usable keyword surface).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Pay 2 life: Remove a counter from target nonland permanent.".into(),
            cost: ActivationCost {
                tap: true,
                life: 2,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: remove_a_counter,
        }),
    )
}

fn remove_a_counter(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Remove a counter" of ANY kind is not expressible —
    // Effect::RemoveCounters requires a specific CounterKind and there
    // is no "remove one counter of the controller's choice" primitive.
    Vec::new()
}
