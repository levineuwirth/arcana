//! Invasion of Shandalar // Leyline Surge — `{3}{G}{G}` Battle — Siege.
//! Enters with defense counters. When this Siege enters, return up to three target permanent
//! cards from your graveyard to your hand.
//! Back face (Leyline Surge): Enchantment — "At the beginning of your upkeep, you may put a
//! permanent card from your hand onto the battlefield."
//!
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11).
//! GAP: "up to three targets" multi-target return-from-graveyard — each target is returned
//!      individually via the targets vec.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Shandalar");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Leyline Surge — Enchantment (no mana cost on back)
    let back_name = reg.interner_mut().intern("Leyline Surge");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::ENCHANTMENT.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 6,
            })
            .with_transform_back(back)
            // "When this Siege enters, return up to three target permanent cards from your
            // graveyard to your hand."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_return_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::permanent(),
                    },
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
            })
            // Back face (Leyline Surge): "At the beginning of your upkeep, you
            // may put a permanent card from your hand onto the battlefield."
            // Face-gated to the back face (CR 712).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_put_permanent_from_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1),
        // GAP: defeat→cast-back-face not auto-wired (CR 310.11).
    )
}

fn upkeep_put_permanent_from_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may put a permanent card from your hand onto the battlefield" —
    // optional pick over the controller's hand.
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types_any(TypeLine(
            TypeLine::CREATURE
                | TypeLine::ENCHANTMENT
                | TypeLine::ARTIFACT
                | TypeLine::LAND
                | TypeLine::PLANESWALKER
                | TypeLine::BATTLE,
        )),
        tapped: false,
    }]
}

fn etb_return_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::ReturnFromGraveyardToHand { target: *id })
            } else {
                None
            }
        })
        .collect()
}
