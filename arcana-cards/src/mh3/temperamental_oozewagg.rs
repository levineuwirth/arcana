//! Temperamental Oozewagg — `{3}{G}` 4/4 Ooze Brushwagg.
//!
//! `{2}{G}: Adapt 2.` (If this creature has no +1/+1 counters on it,
//! put two +1/+1 counters on it.)
//! "Modified creatures you control have trample." (static — GAP'd)
//!
//! Adapt is not in the usable keyword surface (empty `keywords`). The
//! activated ability puts two +1/+1 counters on this creature; the
//! "only if it has no +1/+1 counters" gate has no in-effect condition
//! primitive, so it is a documented fidelity GAP (the counters are
//! added unconditionally). The Modified-creatures-have-trample static
//! is a continuous ability and is GAP'd.

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
    let name = reg.interner_mut().intern("Temperamental Oozewagg");
    let ooze = reg.interner_mut().intern("Ooze");
    let brushwagg = reg.interner_mut().intern("Brushwagg");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);
    subtypes.0.insert(brushwagg);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP (static): "Modified creatures you control have trample" — continuous ability.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{G}: Adapt 2.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
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
    )
}

fn adapt_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP (fidelity): the "if it has no +1/+1 counters" Adapt gate has no in-effect condition primitive.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
