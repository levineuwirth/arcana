//! Draconautics Engineer — `{1}{R}` 2/2 Creature — Goblin Artificer.
//! Exhaust — {R}: Other creatures you control gain haste until end of
//!   turn. Put a +1/+1 counter on this creature.
//! Exhaust — {3}{R}: Create a 4/4 red Dinosaur Dragon creature token
//!   with flying.
//!
//! GAP: Exhaust is not an expressible KeywordAbility, and there is no
//! "activate only once [per game]" ActivationCost gate — the activations
//! are emitted as ordinary mana abilities without the once-only limit.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Draconautics Engineer");
    let goblin = reg.interner_mut().intern("Goblin");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Exhaust is not an expressible KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {R}: Other creatures you control gain haste until end of turn. Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: haste_and_counter,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {3}{R}: Create a 4/4 red Dinosaur Dragon creature token with flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_dragon,
            }),
    )
}

fn haste_and_counter(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids: Vec<_> = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    )
    .into_iter()
    .filter(|id| *id != ctx.source)
    .collect();
    vec![Effect::Sequence(vec![
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::GrantKeyword {
                target: NULL_OBJECT_ID,
                keyword: KeywordAbility::Haste,
                duration: Duration::EndOfTurn,
            }),
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ])]
}

fn make_dragon(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dino = reg.interner().lookup("Dinosaur").unwrap_or_default();
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let mut tok_subtypes = SubtypeSet::default();
    tok_subtypes.0.insert(dino);
    tok_subtypes.0.insert(dragon);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: dragon,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: tok_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
