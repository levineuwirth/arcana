//! Kiora, Master of the Depths — `{2}{G}{U}` Legendary Planeswalker — Kiora,
//! starting loyalty 4.
//!
//! +1: Untap up to one target creature and up to one target land.
//! −2: Reveal the top four cards of your library. You may put a creature card
//!     and/or a land card from among them into your hand. Put the rest into
//!     your graveyard. GAP: a two-independent-pick reveal (creature AND/OR
//!     land) isn't expressible via the single-pick DigTopN.
//! −8: You get an emblem with "Whenever a creature you control enters, you may
//!     have it fight target creature." Then create three 8/8 blue Octopus
//!     creature tokens. (The three tokens are minted; the emblem is GAP.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kiora, Master of the Depths");
    let kiora = reg.interner_mut().intern("Kiora");
    let _octopus = reg.interner_mut().intern("Octopus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kiora);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap up to one target creature and up to one target \
                       land.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .with_types(TypeLine::LAND.into()),
                        ),
                        count: TargetCount::UpTo(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Reveal the top four cards of your library. You may put \
                       a creature card and/or a land card from among them into \
                       your hand. Put the rest into your graveyard.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-8: You get an emblem with \"Whenever a creature you \
                       control enters, you may have it fight target creature.\" \
                       Then create three 8/8 blue Octopus creature tokens.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight,
            }),
    )
}

fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|c| match c {
            TargetChoice::Object(id) => Some(Effect::Untap { target: *id }),
            _ => None,
        })
        .collect()
}

fn minus_two(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: a two-independent-pick reveal (a creature AND/OR a land) isn't
    // expressible via the single-pick DigTopN.
    Vec::new()
}

fn minus_eight(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with a fight-on-creature-enter ability. The three 8/8 blue
    // Octopus tokens are minted.
    let octopus = reg.interner().lookup("Octopus")
        .expect("Octopus interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(octopus);
    let token = TokenDefinition {
        name: octopus,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token: token.clone() },
        Effect::CreateToken { controller: ctx.controller, token },
    ]
}
