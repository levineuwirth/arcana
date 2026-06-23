//! Vampire Hexmage — `{B}{B}` 2/1 Creature — Vampire Shaman.
//!
//! Oracle:
//! * First strike.
//! * Sacrifice this creature: Remove all counters from target permanent.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vampire Hexmage");
    let vampire = reg.interner_mut().intern("Vampire");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice this creature: Remove all counters from target permanent.".into(),
            cost: ActivationCost {
                sacrifice: true,
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
            effect: remove_all_counters,
        }),
    )
}

fn remove_all_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Remove all counters from target permanent." RemoveCounters is per-kind
    // and caps at the actual count present, so we clear each common counter
    // kind with a large request. Fidelity gap: an arbitrary Named counter not
    // listed here would not be cleared.
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    let kinds = [
        CounterKind::PlusOnePlusOne,
        CounterKind::MinusOneMinusOne,
        CounterKind::Loyalty,
        CounterKind::Charge,
        CounterKind::Time,
        CounterKind::Fade,
        CounterKind::Quest,
        CounterKind::Study,
        CounterKind::Poison,
        CounterKind::Energy,
        CounterKind::Shield,
        CounterKind::Stun,
        CounterKind::Lore,
        CounterKind::Defense,
        CounterKind::Level,
    ];
    vec![Effect::Sequence(
        kinds
            .iter()
            .map(|&kind| Effect::RemoveCounters {
                target: id,
                kind,
                count: u32::MAX,
            })
            .collect(),
    )]
}
