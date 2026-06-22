//! Tuskguard Captain — `{2}{G}` 2/3 Creature — Human Warrior.
//! Outlast {G} ({G}, {T}: Put a +1/+1 counter on this creature. Outlast only
//! as a sorcery.)
//! "Each creature you control with a +1/+1 counter on it has trample."
//!
//! Decomposition:
//! - Keyword line: Outlast — NOT in the usable keyword surface, so
//!   `keywords: vec![]`. The outlast ability ({G}, {T}: +1/+1 counter,
//!   sorcery speed) is modeled below as an activated ability.
//! - "Each creature you control with a +1/+1 counter on it has trample." is a
//!   pure static continuous ability — GAP (no expressible Effect).

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
    let name = reg.interner_mut().intern("Tuskguard Captain");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword `Outlast` not in usable KeywordAbility surface; the
        // outlast activation is modeled below as an activated ability.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Each creature you control with a +1/+1 counter on it has
    // trample" — a continuous static ability with no expressible Effect.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{G}, {T}: Put a +1/+1 counter on this creature. Outlast only as a sorcery."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{G}").expect("valid cost"),
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

fn outlast_counter(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
