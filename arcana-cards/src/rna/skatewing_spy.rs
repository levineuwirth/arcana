//! Skatewing Spy — `{3}{U}` 2/3 Creature — Vedalken Rogue Mutant.
//!
//! * `{5}{U}: Adapt 2.` (If this creature has no +1/+1 counters on it, put two
//!   +1/+1 counters on it.) Adapt is not in the usable keyword surface, so it
//!   is wired as its activated body: `{5}{U}` puts two +1/+1 counters on this
//!   creature. GAP: the "only if it has no +1/+1 counters" Adapt precondition
//!   is not expressible on an activated ability with the demonstrated surface,
//!   so the counters are added unconditionally.
//! * Each creature you control with a +1/+1 counter on it has flying.
//!   (Pure continuous static — GAP'd; not a triggered/activated ability.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skatewing Spy");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let rogue = reg.interner_mut().intern("Rogue");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(rogue);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP (static): "Each creature you control with a +1/+1 counter on it has
    // flying." — a pure continuous keyword-granting static, not a
    // triggered/activated ability.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{U}: Adapt 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{U}").expect("valid cost"),
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

fn adapt_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Adapt's "only if it has no +1/+1 counters" gate is not expressible
    // here; the two +1/+1 counters are added unconditionally.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
