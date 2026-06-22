//! Bontu the Glorified — `{2}{B}` 4/6 legendary God with Menace and
//! Indestructible.
//! "Bontu can't attack or block unless a creature died under your control
//!  this turn." (a continuous restriction static — GAP'd)
//! "{1}{B}, Sacrifice another creature: Scry 1. Each opponent loses 1 life
//!  and you gain 1 life."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bontu the Glorified");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    // Scryfall lists "Scry" among keywords, but it is part of the
    // activated ability's effect, not a base KeywordAbility variant.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Menace, KeywordAbility::Indestructible],
        ..Default::default()
    };

    // GAP: "Bontu can't attack or block unless a creature died under your
    // control this turn" — a continuous attack/block restriction static;
    // not expressible as a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{B}, Sacrifice another creature: Scry 1. Each opponent loses 1 life and you gain 1 life.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                sacrifice_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: scry_drain,
        }),
    )
}

fn scry_drain(state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::Scry {
        player: ctx.controller,
        count: 1,
    }];
    // Each opponent loses 1 life.
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::LoseLife {
            player: opp,
            amount: 1,
        });
    }
    effects.push(Effect::GainLife {
        player: ctx.controller,
        amount: 1,
    });
    effects
}
