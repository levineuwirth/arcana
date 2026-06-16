//! Ersta, Friend to All — `{W}{U}{B}{R}{G}` Legendary Planeswalker — Ersta, starting loyalty 6.
//! +1: Create a 1/1 Human Wizard creature token that's all colors.
//! −3: Choose a card name from a list, create a copy with that name, may cast it for free. —
//!     choose-card-name + create-copy not expressible → GAP'd (shell with correct −3 cost).
//! −8: Emblem — TRIGGERED "at the beginning of your upkeep, if you control 20+ Wizards, you win
//!     the game." Win-the-game is not buildable → emit CreateEmblem with empty abilities + GAP.
//! ("can be your commander" — ignored, not a loyalty ability.)

use arcana_core::effects::{Effect, EmblemDefinition, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ersta, Friend to All");
    let sub = reg.interner_mut().intern("Ersta");
    let _human = reg.interner_mut().intern("Human");
    let _wizard = reg.interner_mut().intern("Wizard");
    let _emblem = reg.interner_mut().intern("Ersta, Friend to All emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 Human Wizard creature token that's all colors.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Choose a card name from among Enlightened Tutor, Mystical Tutor, \
                       Booster Tutor, Imperial Recruiter, and Worldly Tutor. Create a copy of \
                       the card with the chosen name. You may cast the copy without paying its \
                       mana cost."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: You get an emblem with \"At the beginning of your upkeep, if you \
                       control twenty or more Wizards, you win the game.\""
                    .into(),
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

/// `+1: Create a 1/1 Human Wizard creature token that's all colors.`
fn plus_one_token(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").expect("Human interned");
    let wizard = reg.interner().lookup("Wizard").expect("Wizard interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(human);
    token_subtypes.0.insert(wizard);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: wizard,
            colors: ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red()
                | ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `−3` — GAP: choose-a-card-name + create-copy + cast-for-free is not expressible.
fn minus_three(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}

/// `−8: emblem.` GAP: "if you control 20+ Wizards, you win the game" is not buildable
/// (no win-the-game effect). Emit CreateEmblem with empty abilities to record the emblem.
fn minus_eight(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let emblem_name = reg
        .interner()
        .lookup("Ersta, Friend to All emblem")
        .expect("emblem name interned");
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: vec![],
            abilities: vec![],
        },
    }]
}
