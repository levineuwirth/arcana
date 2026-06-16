//! Incubation Druid — `{1}{G}` 0/2 Elf Druid.
//! {T}: Add one mana of any type a land you control could produce (three if it
//! has a +1/+1 counter) — GAP, "any type a land could produce" is not an
//! expressible mana production. {3}{G}{G}: Adapt 3 (put three +1/+1 counters
//! on it if it has none).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Incubation Druid");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    // GAP: "{T}: Add one mana of any type that a land you control could
    // produce. If this creature has a +1/+1 counter on it, add three mana of
    // that type instead." — "any type a land you control could produce" is not
    // an expressible mana production (no such Effect::AddMana form).
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{G}{G}: Adapt 3.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{G}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: adapt_three,
        }),
    )
}

fn adapt_three(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Adapt 3: only put counters on if it currently has no +1/+1 counters.
    let has_counters = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne))
        > 0;
    if has_counters {
        Vec::new()
    } else {
        vec![Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 3,
        }]
    }
}
