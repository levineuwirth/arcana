//! Lesser Werewolf — `{3}{B}` 2/4 black Werewolf.
//! "{B}: If this creature's power is 1 or more, it gets -1/-0 until end of
//! turn and put a -0/-1 counter on target creature blocking or blocked by
//! this creature. Activate only during the declare blockers step."
//!
//! "Target creature blocking or blocked by this creature" is wired via
//! `TargetFilter::CreatureBlockingOrBlockedBySource` (source-relative pairing).
//! GAP: "-0/-1 counter" — CounterKind::MinusOneMinusOne would be used but
//! the effect is "-0/-1" which is unusual; the counter half is omitted.
//! GAP: "If this creature's power is 1 or more" precondition and the
//! "Activate only during the declare blockers step" timing window are
//! not enforced.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lesser Werewolf");
    let werewolf = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(werewolf);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: If this creature's power is 1 or more, it gets -1/-0 until end of turn and put a -0/-1 counter on target creature blocking or blocked by this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::CreatureBlockingOrBlockedBySource,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_effect,
            }),
    )
}

fn pump_effect(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "-0/-1 counter" on the target not expressible (no -0/-1
    // CounterKind with a P/T layer effect); only the self -1/-0 pump is emitted.
    vec![Effect::Pump {
        target: ctx.source,
        power: -1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
