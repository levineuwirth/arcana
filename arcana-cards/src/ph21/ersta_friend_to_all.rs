//! Ersta, Friend to All — `{W}{U}{B}{R}{G}` legendary planeswalker, starting loyalty 5.
//!
//! +1: Create a 1/1 Human Wizard creature token that's all colors.
//! −3: Choose a tutor card name, create a copy, cast it free (GAP).
//! −8: You get an emblem (GAP).
//!
//! Scope: +1 token creation is fully expressed (a five-color 1/1 Human
//! Wizard). The −3 "choose a card name from a fixed list, create+cast a
//! copy" has no demonstrated Effect. The −8 emblem with a "you win the
//! game" upkeep trigger is GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, Color, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ersta, Friend to All");
    let ersta = reg.interner_mut().intern("Ersta");
    // Interned here so the +1 resolver can look them up via the
    // non-mut interner at resolution time.
    let _token_name = reg.interner_mut().intern("Human Wizard");
    let _human = reg.interner_mut().intern("Human");
    let _wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ersta);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black()
            | ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a 1/1 Human Wizard creature token that's all \
                       colors.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_token,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Choose a card name from among Enlightened Tutor, \
                       Mystical Tutor, Booster Tutor, Imperial Recruiter, and \
                       Worldly Tutor. Create a copy of the card with the chosen \
                       name. You may cast the copy without paying its mana cost.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_tutor_copy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: You get an emblem with \"At the beginning of your \
                       upkeep, if you control twenty or more Wizards, you win \
                       the game.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight_emblem,
            }),
    )
}

/// `+1: Create a 1/1 all-colors Human Wizard token.`
fn plus_one_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg.interner().lookup("Human Wizard").expect("interned");
    let human = reg.interner().lookup("Human").expect("interned");
    let wizard = reg.interner().lookup("Wizard").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let colors = ColorSet::colorless()
        .with(Color::White).with(Color::Blue).with(Color::Black)
        .with(Color::Red).with(Color::Green);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: token_name,
            colors,
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `−3` — choose a fixed tutor name, copy + cast free.
fn minus_three_tutor_copy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: choose-card-name-from-fixed-list + create copy + cast free.
    Vec::new()
}

/// `−8` — emblem.
fn minus_eight_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem with conditional "you win the game" upkeep trigger.
    Vec::new()
}
