//! Blitzwing, Cruel Tormentor // Blitzwing, Adaptive Assailant — `{5}{B}` Legendary Artifact Creature — Robot 6/5.
//!
//! Front face:
//! More Than Meets the Eye {3}{B} (alternative cast cost — GAP: alt cast not modeled)
//! At the beginning of your end step, target opponent loses life equal to the life that player
//! lost this turn. If no life is lost this way, convert Blitzwing.
//!
//! Back face (Legendary Artifact — Vehicle):
//! Living metal (During your turn, this Vehicle is also a creature.) — GAP: not expressible.
//! At the beginning of combat on your turn, choose flying or indestructible at random.
//! Blitzwing gains that ability until end of turn.
//! Whenever Blitzwing deals combat damage to a player, convert it.
//!
//! GAP: Keywords "Living metal", "Convert", "More Than Meets the Eye" — not in keyword list.
//! GAP: Front trigger "life that player lost this turn" — no per-player-per-turn life-loss
//! tracking in script API; trigger effect returns Vec::new().
//! GAP: Back trigger "choose flying or indestructible at random" — random keyword grant not
//! expressible; using FlipCoin to approximate (Heads → Flying, Tails → Indestructible).
//! GAP: Back face is Artifact — Vehicle (not creature), no P/T — modeled with P/T from front.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blitzwing, Cruel Tormentor");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // Back face — Blitzwing, Adaptive Assailant (Legendary Artifact — Vehicle)
    let back_name = reg.interner_mut().intern("Blitzwing, Adaptive Assailant");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle_sub);

    let back_chars = Characteristics {
        name: back_name,
        colors: ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: back_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // Vehicle form — no printed P/T; using front face values as placeholder
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let back_face = CardFace {
        name: back_name,
        characteristics: back_chars,
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back_face)
            // Front face trigger: at beginning of your end step, target opponent loses life
            // equal to life that player lost this turn. If none, convert Blitzwing.
            // GAP: "life that player lost this turn" not accessible via script API.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            })
            // Back face trigger (id 2): at beginning of combat on your turn, flip coin for ability.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::BeginCombat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_begin_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face trigger (id 3): whenever Blitzwing deals combat damage to a player,
            // convert it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: convert_to_front,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 1 (front end step) fires only on the front face; the begin-combat
            // random-keyword trigger (2) and the combat-damage convert trigger (3) only on back.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1),
    )
}

fn end_step_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(_p) = target else { return Vec::new(); };
    // GAP: "loses life equal to the life that player lost this turn" — per-player-per-turn
    // life-loss tracking not available in script API.
    // GAP: "If no life is lost this way, convert Blitzwing" — conditional on effect result
    // not expressible.
    Vec::new()
}

fn convert_to_front(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn combat_begin_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Choose flying or indestructible at random" — approximate with FlipCoin
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        }),
        lose: Some(Box::new(Effect::GrantKeyword {
            target: trig.source,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        })),
    }]
}
