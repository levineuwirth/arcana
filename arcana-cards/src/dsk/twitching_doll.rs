//! Twitching Doll — `{1}{G}` 2/2 Artifact Creature — Spider Toy.
//!
//! * `{T}: Add one mana of any color. Put a nest counter on this creature.`
//! * `{T}, Sacrifice this creature: Create a 2/2 green Spider creature token
//!   with reach for each counter on this creature. Activate only as a sorcery.`

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Twitching Doll");
    let spider = reg.interner_mut().intern("Spider");
    let toy = reg.interner_mut().intern("Toy");
    let _nest = reg.interner_mut().intern("nest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(toy);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color. Put a nest counter on this creature."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_mana_and_nest,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice this creature: Create a 2/2 green Spider creature \
                       token with reach for each counter on this creature. Activate only \
                       as a sorcery."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_spiders,
            }),
    )
}

fn add_mana_and_nest(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Add one mana of any color" — no any-color mana primitive in the
    // demonstrated API (only ManaUnit::plain(fixed-color)). The nest counter
    // half is expressible and emitted.
    let nest = reg.interner().lookup("nest");
    match nest {
        Some(n) => vec![Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Named(n),
            count: 1,
        }],
        None => Vec::new(),
    }
}

fn make_spiders(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let count = state
        .objects
        .get(ctx.source)
        .map_or(0, |o| {
            let nest = reg.interner().lookup("nest").map_or(0, |n| o.count_counters(CounterKind::Named(n)));
            nest
        });
    let spider = match reg.interner().lookup("Spider") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    let mut out = Vec::new();
    for _ in 0..count {
        out.push(Effect::CreateToken {
            controller: ctx.controller,
            token: TokenDefinition {
                name: spider,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: subtypes.clone(),
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![KeywordAbility::Reach],
                abilities: vec![],
            },
        });
    }
    out
}
