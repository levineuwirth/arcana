//! Enduring Sliver — `{1}{W}` 2/2 Creature — Sliver.
//!
//! Outlast {2} ({2}, {T}: Put a +1/+1 counter on this creature. Outlast only
//! as a sorcery.)
//! Other Sliver creatures you control have outlast {2}.
//!
//! # Decomposition
//! * Outlast {2} is not a `KeywordAbility` variant in the demonstrated surface,
//!   so it is wired as its expanded activated ability: `{2}, {T}: put a +1/+1
//!   counter on this creature` (id-less, sorcery speed). The "only as a
//!   sorcery" timing is honored via `is_instant_speed: false`.
//! * GAP: "Other Sliver creatures you control have outlast {2}" — a continuous
//!   ability-granting static (grant an activated ability to other permanents)
//!   is not expressible with the demonstrated primitives.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enduring Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "Other Sliver creatures you control have outlast {2}" — static
            // ability grant to other permanents is not expressible.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Put a +1/+1 counter on this creature. Outlast only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: outlast_counter,
            }),
    )
}

fn outlast_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
