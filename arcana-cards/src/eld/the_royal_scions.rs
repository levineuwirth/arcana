//! The Royal Scions — `{1}{U}{R}` Legendary Planeswalker — Will Rowan.
//! Printed starting loyalty 5 (CR 113.3c).
//!
//! Loyalty abilities (CR 606):
//! * `+1`: Draw a card, then discard a card.
//! * `+1`: Target creature gets +2/+0 and gains first strike and trample
//!   until end of turn.
//! * `−8`: Draw four cards. When you do, The Royal Scions deals damage to
//!   any target equal to the number of cards in your hand.
//!
//! Subtype: this PW has two name-words; both "Will" and "Rowan" are
//! interned as subtypes.
//!
//! Scope: the two `+1`s are fully expressible. The `−8`'s draw-four is
//! expressible; the "when you do, deal damage equal to cards in hand"
//! reflexive trigger with a dynamic damage amount is GAP'd (the draw and
//! the damage clause are linked by a reflexive trigger + a count-cards
//! amount that the demonstrated activated-ability surface can't bind).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Royal Scions");
    let will = reg.interner_mut().intern("Will");
    let rowan = reg.interner_mut().intern("Rowan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(will);
    subtypes.0.insert(rowan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw a card, then discard a card.".into(),
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
                effect: plus_one_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Target creature gets +2/+0 and gains first strike and \
                       trample until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_pump,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: Draw four cards. When you do, The Royal Scions deals \
                       damage to any target equal to the number of cards in your \
                       hand.".into(),
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
                effect: minus_eight_draw,
            }),
    )
}

use arcana_core::effects::DiscardChoice;

/// `+1`: draw then discard.
fn plus_one_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

/// `+1`: +2/+0, first strike + trample until end of turn.
fn plus_one_pump(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Trample],
    }]
}

/// `−8`: draw four; the reflexive damage-equal-to-hand rider is a GAP.
fn minus_eight_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "When you do, deal damage to any target equal to the number of
    // cards in your hand" is a reflexive trigger with a dynamic
    // count-cards amount and a target chosen at the trigger — not
    // expressible from this activated-ability resolver. Draw-four emitted.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 4,
    }]
}
