//! Teyo, Geometric Tactician — `{2}{W}` Legendary Planeswalker — Teyo,
//! starting loyalty 5. Mono-white.
//!
//! Oracle:
//! When Teyo enters, create a 0/4 white Wall creature token with defender and
//!   flying.
//! +1: You and target opponent each draw a card.
//! −2: Choose left or right. Until your next turn, each player may attack only
//!     the nearest opponent in the last chosen direction and planeswalkers
//!     controlled by that opponent.
//!
//! # Scope
//! * ETB — modeled: create a 0/4 white Wall token with defender and flying.
//! * +1 — modeled: you and target opponent each draw a card.
//! * −2 — GAP: "choose left/right" attack-direction restriction is a bespoke
//!   multiplayer combat rule not expressible.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teyo, Geometric Tactician");
    let sub = reg.interner_mut().intern("Teyo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);
    let _ = reg.interner_mut().intern("Wall");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_wall,
                trigger_zones: vec![],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: You and target opponent each draw a card.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_draw,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Choose left or right. Until your next turn, each \
                       player may attack only the nearest opponent in the last \
                       chosen direction and planeswalkers controlled by that \
                       opponent.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_gap,
            }),
    )
}

/// ETB: create a 0/4 white Wall token with defender and flying.
fn etb_wall(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wall = reg.interner().lookup("Wall").unwrap_or_default();
    let mut t_subtypes = SubtypeSet::default();
    t_subtypes.0.insert(wall);
    let token = TokenDefinition {
        name: wall,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: t_subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender, KeywordAbility::Flying],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

/// `+1:` you and target opponent each draw a card.
fn plus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }];
    if let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() {
        effects.push(Effect::DrawCards {
            player: *p,
            count: 1,
        });
    }
    effects
}

/// `−2` — GAP: bespoke "choose left/right" attack-direction restriction.
fn minus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: choose-left-or-right multiplayer combat restriction.
    Vec::new()
}
