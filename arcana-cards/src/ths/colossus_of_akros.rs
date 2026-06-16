//! Colossus of Akros — `{8}` 10/10 Artifact Creature — Golem with Defender
//! and Indestructible.
//!
//! Oracle:
//! * Defender, indestructible.
//! * {10}: Monstrosity 10. (put ten +1/+1 counters on it and it becomes
//!   monstrous — the "becomes monstrous" flag isn't modeled; counters are.)
//! * As long as this creature is monstrous, it has trample and can attack as
//!   though it didn't have defender. (static — GAP, depends on monstrous flag)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Colossus of Akros");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(10)),
        toughness: Some(PtValue::Fixed(10)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: static "As long as monstrous, it has trample and can attack as
    // though it didn't have defender" depends on the unmodeled monstrous flag.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{10}: Monstrosity 10.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{10}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: monstrosity_ten,
        }),
    )
}

fn monstrosity_ten(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // Monstrosity's "becomes monstrous" flag isn't modeled; the ten +1/+1
    // counters are applied. (The if-not-already-monstrous gate is unmodeled.)
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 10,
    }]
}
