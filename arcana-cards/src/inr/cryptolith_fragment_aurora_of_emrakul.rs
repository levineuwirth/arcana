//! Cryptolith Fragment // Aurora of Emrakul (transforming DFC, layout "transform")
//!
//! Front face: Cryptolith Fragment — {3} Artifact.
//!   This artifact enters tapped.
//!   {T}: Add one mana of any color. Each player loses 1 life.
//!   At the beginning of your upkeep, if each player has 10 or less life, transform this
//!     artifact.
//! Back face: Aurora of Emrakul — Creature — Eldrazi Reflection, 5/5, Flying, Deathtouch.
//!   Whenever this creature attacks, each opponent loses 3 life.
//!
//! GAP: "Add one mana of any color" — Effect::AddMana requires a fixed color; modeled as
//!   colorless (the color choice is not expressible).
//! Note: the front-face {T} ability is NOT a pure mana ability (it also drains every
//!   player 1 life), so it is authored with is_mana_ability: false.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, ManaColor, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cryptolith Fragment");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let reflection = reg.interner_mut().intern("Reflection");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Aurora of Emrakul");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(eldrazi);
    back_subtypes.0.insert(reflection);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Deathtouch],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            // This artifact enters tapped.
            .with_enters_with(EntersWithSpec::Tapped)
            .with_transform_back(back)
            // {T}: Add one mana of any color. Each player loses 1 life.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color. Each player loses 1 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").expect("valid cost"),
                    tap: true,
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: tap_for_mana_and_drain,
            })
            // Front-face: at the beginning of your upkeep, if each player has 10 or less
            // life, transform this artifact.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_each_player_ten_or_less),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0)
            // Back-face: whenever this creature attacks, each opponent loses 3 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1),
    )
}

fn tap_for_mana_and_drain(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "any color" choice not available; emitting colorless as placeholder.
    let mut effects = vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }];
    for p in script::all_players(state) {
        effects.push(Effect::LoseLife {
            player: p,
            amount: 1,
        });
    }
    effects
}

fn if_each_player_ten_or_less(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    script::all_players(s)
        .into_iter()
        .all(|p| script::life(s, p) <= 10)
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform {
        target: trig.source,
    }]
}

fn attack_drain(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::LoseLife {
            player: p,
            amount: 3,
        })
        .collect()
}
