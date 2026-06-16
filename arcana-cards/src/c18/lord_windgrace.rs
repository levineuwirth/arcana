//! Lord Windgrace — `{2}{B}{R}{G}` Legendary Planeswalker — Windgrace.
//! Printed starting loyalty 5 (CR 113.3c). Colors B/R/G.
//!
//! "Lord Windgrace can be your commander" is a command-zone eligibility
//! static, not a loyalty ability — GAP.
//!
//! Loyalty abilities (CR 606):
//! * `+2`: Discard a card, then draw a card. If a land card is discarded
//!   this way, draw an additional card. — discard-then-draw is emitted;
//!   the "if a land discarded, draw additional" half is GAP'd (the
//!   discard contents aren't inspectable from this resolver).
//! * `−3`: Return up to two target land cards from your graveyard to the
//!   battlefield. — GAP (graveyard-targeting needs a concrete
//!   `Zone::Graveyard(player)`; no any-graveyard target sentinel).
//! * `−11`: Destroy up to six target nonland permanents, then create six
//!   2/2 green Cat Warrior creature tokens with forestwalk. — both halves
//!   expressible.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

use arcana_core::effects::DiscardChoice;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lord Windgrace");
    let windgrace = reg.interner_mut().intern("Windgrace");
    let _cat = reg.interner_mut().intern("Cat");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(windgrace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Discard a card, then draw a card. If a land card is \
                       discarded this way, draw an additional card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Return up to two target land cards from your graveyard \
                       to the battlefield.".into(),
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
                effect: minus_three_lands,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−11: Destroy up to six target nonland permanents, then \
                       create six 2/2 green Cat Warrior creature tokens with \
                       forestwalk.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 11)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::Any),
                    ),
                    count: TargetCount::UpTo(6),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eleven,
            }),
    )
}

/// `+2`: discard then draw; the land-conditional extra draw is a GAP.
fn plus_two_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if a land card is discarded this way, draw an additional card"
    // depends on the discarded card's type, which this resolver can't
    // inspect. The base discard-then-draw is emitted.
    vec![
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
    ]
}

/// `−3`: graveyard land reanimation — GAP.
fn minus_three_lands(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "target land cards in your graveyard" needs a concrete
    // Zone::Graveyard target; no any-graveyard target sentinel in the
    // demonstrated surface.
    Vec::new()
}

/// `−11`: destroy up to six nonland permanents, then make six Cat Warriors.
fn minus_eleven(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = ctx
        .targets
        .targets
        .iter()
        .filter_map(|c| match c {
            TargetChoice::Object(id) => Some(Effect::DestroyPermanent { target: *id }),
            _ => None,
        })
        .collect();

    let cat = reg
        .interner()
        .lookup("Cat")
        .expect("Cat interned during register()");
    let warrior = reg
        .interner()
        .lookup("Warrior")
        .expect("Warrior interned during register()");
    let forest = reg
        .interner()
        .lookup("Forest")
        .expect("Forest interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: cat,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Landwalk(forest)],
        abilities: vec![],
    };
    for _ in 0..6 {
        effects.push(Effect::CreateToken {
            controller: ctx.controller,
            token: token.clone(),
        });
    }
    effects
}
