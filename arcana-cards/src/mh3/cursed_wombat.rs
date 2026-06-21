//! Cursed Wombat — `{B}{G}` 2/3 Nightmare Wombat.
//!
//! Oracle:
//! * `{2}{B}{G}: Adapt 2.` (If this creature has no +1/+1 counters on it,
//!   put two +1/+1 counters on it.)
//! * Permanents you control have "Whenever one or more +1/+1 counters are put
//!   on this permanent, put an additional +1/+1 counter on it. This ability
//!   triggers only once each turn." — a static ability-granting effect.
//!
//! The activated Adapt ability is expressible only as an unconditional
//! "put two +1/+1 counters on this creature"; the "if no +1/+1 counters" gate
//! that defines Adapt is a resolution-time precondition with no expressible
//! primitive, so the ability is GAP'd rather than emitted as a wrong
//! (always-fires) variant. The static permanent-granting line is also GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cursed Wombat");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let wombat = reg.interner_mut().intern("Wombat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(wombat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}{G}: Adapt 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: adapt_two,
            }),
        // GAP: static — "Permanents you control have 'Whenever one or more
        // +1/+1 counters are put on this permanent, put an additional +1/+1
        // counter on it. This ability triggers only once each turn.'": granting
        // a per-turn-limited triggered ability to every permanent you control is
        // not expressible with the available continuous/static primitives.
    )
}

fn adapt_two(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Adapt 2 — "if this creature has no +1/+1 counters on it, put two
    // +1/+1 counters on it." There is no Effect::Adapt and no expressible
    // resolution-time "only if no counters present" gate; emitting an
    // unconditional AddCounters would be a materially wrong card, so the whole
    // effect is GAP'd.
    Vec::new()
}
