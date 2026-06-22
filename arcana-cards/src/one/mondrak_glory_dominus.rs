//! Mondrak, Glory Dominus — `{2}{W}{W}` 4/4 Legendary Phyrexian Horror (white).
//!
//! * GAP (static): "If one or more tokens would be created under your
//!   control, twice that many of those tokens are created instead." — a
//!   token-doubling replacement effect, not a triggered/activated ability
//!   and not expressible here.
//! * "{1}{W/P}{W/P}, Sacrifice two other artifacts and/or creatures: Put an
//!   indestructible counter on Mondrak." → an activated ability. The mana
//!   cost ({1}{W/P}{W/P}) and the "sacrifice two other artifacts and/or
//!   creatures" cost (sacrifice_other over an artifact-or-creature filter,
//!   count 2 — the engine always excludes this source) are expressed; the
//!   effect adds a Named "indestructible" counter to this creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mondrak, Glory Dominus");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let _indestructible = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{W/P}{W/P}, Sacrifice two other artifacts and/or \
                   creatures: Put an indestructible counter on Mondrak."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{W/P}{W/P}").expect("valid cost"),
                sacrifice_other: Some(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                ),
                sacrifice_other_count: 2,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_indestructible_counter,
        }),
    )
}

fn add_indestructible_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("indestructible").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind,
        count: 1,
    }]
}
