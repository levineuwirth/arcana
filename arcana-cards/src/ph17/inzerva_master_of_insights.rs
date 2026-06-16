//! Inzerva, Master of Insights — `{1}{2/U}{2/R}` Legendary Planeswalker —
//! Inzerva, starting loyalty 5. Colors R/U.
//!
//! +2: Draw two cards, then discard a card.
//! −2: Look at the top two cards of each other player's library, then put any
//!   number on the bottom and the rest on top in any order. Scry 2. The Scry 2 is
//!   modeled (`Effect::Scry`); GAP: "look at the top two of each OTHER player's
//!   library and reorder" has no demonstrated surface (only your own scry/surveil).
//! −4: emblem with TWO clauses. "Your opponents play with their hands revealed"
//!   is a rule-altering static — GAP (no anthem/keyword builder). "Whenever an
//!   opponent draws a card, this emblem deals 1 damage to them" IS modeled as a
//!   triggered emblem ability (CardDrawn(Opponent) → DealDamage to the drawer).

use arcana_core::effects::{Effect, EmblemDefinition, DiscardChoice};
use arcana_core::events::{DamageTarget, GameEvent};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inzerva, Master of Insights");
    let inzerva = reg.interner_mut().intern("Inzerva");
    let _emblem = reg.interner_mut().intern("Inzerva, Master of Insights emblem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(inzerva);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{2/U}{2/R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Draw two cards, then discard a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_loot,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Look at the top two cards of each other player's \
                       library, then put any number of them on the bottom of \
                       that library and the rest on top in any order. Scry \
                       2.".into(),
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
                effect: minus_two_scry,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-4: You get an emblem with \"Your opponents play with \
                       their hands revealed\" and \"Whenever an opponent draws \
                       a card, this emblem deals 1 damage to them.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 4)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_four_emblem,
            }),
    )
}

fn plus_two_loot(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 2 },
        Effect::Discard { player: ctx.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}

fn minus_two_scry(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "look at the top two cards of each other player's library and reorder"
    //      — no demonstrated surface manipulates other players' libraries. Scry 2
    //      (your own) is modeled.
    vec![Effect::Scry { player: ctx.controller, count: 2 }]
}

fn minus_four_emblem(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let emblem_name = reg.interner().lookup("Inzerva, Master of Insights emblem").expect("emblem interned");
    // GAP: "your opponents play with their hands revealed" (rule-altering static).
    //      The "deal 1 damage to an opponent who draws" clause IS modeled.
    vec![Effect::CreateEmblem {
        controller: ctx.controller,
        emblem: EmblemDefinition {
            name: emblem_name,
            statics: Vec::new(),
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: emblem_draw_damage,
                trigger_zones: vec![Zone::Command],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }],
        },
    }]
}

fn emblem_draw_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let GameEvent::DrawCard { player, .. } = trig.trigger_event else { return Vec::new(); };
    vec![Effect::DealDamage {
        source: NULL_OBJECT_ID,
        target: DamageTarget::Player(player),
        amount: 1,
    }]
}
