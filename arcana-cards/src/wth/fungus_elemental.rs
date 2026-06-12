//! Fungus Elemental — `{3}{G}` 3/3 green Fungus Elemental.
//! "{G}, Sacrifice a Forest: Put a +2/+2 counter on this creature. Activate
//! only if this creature entered this turn."
//! The +2/+2 counter is wired as `CounterKind::Named("+2/+2")` plus a
//! permanent +2/+2 continuous effect for its P/T contribution (layer 7d
//! only sums +1/+1 / -1/-1 counters). The "only if this creature entered
//! this turn" gate is enforced via `ActivationCost.activation_condition`.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fungus Elemental");
    let forest = reg.interner_mut().intern("Forest");
    // Interned for the effect fn's lookup of the named counter kind.
    reg.interner_mut().intern("+2/+2");
    let fungus = reg.interner_mut().intern("Fungus");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);
    subtypes.0.insert(elemental);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}, Sacrifice a Forest: Put a +2/+2 counter on this creature. Activate only if this creature entered this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").unwrap(),
                    // "Sacrifice a Forest" — sacrifice a controlled Forest,
                    // not the source.
                    sacrifice_other: Some(
                        arcana_core::targets::ObjectFilter::permanent()
                            .with_subtype_sym(forest),
                    ),
                    // "Activate only if this creature entered this turn."
                    activation_condition: Some(|s, src, _you, _reg| {
                        arcana_core::script::entered_battlefield_this_turn(s, src)
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_two_two_counter,
            }),
    )
}

fn add_two_two_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("+2/+2").map(CounterKind::Named) else {
        return Vec::new();
    };
    // Named "+2/+2" counter plus a permanent +2/+2 continuous effect for
    // its P/T contribution; narrowed GAP: removing the counter later
    // would not remove the P/T boost.
    vec![
        Effect::AddCounters { target: ctx.source, kind, count: 1 },
        Effect::Pump {
            target: ctx.source,
            power: 2,
            toughness: 2,
            duration: Duration::Permanent,
            keywords: vec![],
        },
    ]
}
