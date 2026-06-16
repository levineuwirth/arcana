//! Sanctuary Wall — `{1}{W}` 0/4 white Artifact Creature — Wall with Defender.
//!
//! Oracle:
//! * Defender.
//! * "{2}{W}, {T}: Tap target creature. You may put a stun counter on it. If
//!   you do, put a stun counter on this creature."
//!
//! The activated ability taps the target creature, then puts a stun counter on
//! it and on this creature.
//! GAP: the "You may ... If you do, ..." optionality is not expressible
//! (`OptionalPayment` only models mana/life gates, not a free "may put a
//! counter" choice). We emit the stun counters unconditionally (the "if you do"
//! self-counter is bundled), which is the closest best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanctuary Wall");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![arcana_core::effects::KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}, {T}: Tap target creature. You may put a stun \
                       counter on it. If you do, put a stun counter on this \
                       creature."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_and_stun,
            }),
    )
}

fn tap_and_stun(
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
    // GAP: "you may" optionality on the stun counter is not expressible; the
    // stun counters are emitted unconditionally as the best-effort form.
    vec![
        Effect::Tap { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::Stun,
            count: 1,
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Stun,
            count: 1,
        },
    ]
}
