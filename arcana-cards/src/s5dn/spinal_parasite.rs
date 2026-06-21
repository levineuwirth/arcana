//! Spinal Parasite — `{5}` -1/-1 Artifact Creature — Insect with Sunburst.
//!
//! Sunburst (enters with a +1/+1 counter per color of mana spent to cast it)
//! Remove two +1/+1 counters from this creature: Remove a counter from
//! target permanent.
//!
//! Sunburst is wired as a keyword. The activated ability's cost (remove two
//! +1/+1 counters from this creature) and its target (a permanent) are
//! wired, but the effect "remove a counter" lets the player choose ANY
//! counter kind on the target — `Effect::RemoveCounters` requires a fixed
//! `CounterKind`, so the choose-the-kind effect is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spinal Parasite");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(-1)),
        toughness: Some(PtValue::Fixed(-1)),
        keywords: vec![KeywordAbility::Sunburst],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Remove two +1/+1 counters from this creature: Remove a counter from target permanent.".into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::PlusOnePlusOne, 2)),
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
            effect: remove_a_counter,
        }),
    )
}

fn remove_a_counter(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Remove a counter from target permanent" — the player chooses any
    // counter KIND on the target; Effect::RemoveCounters requires a fixed
    // CounterKind, so the choose-the-kind effect is not expressible.
    Vec::new()
}
