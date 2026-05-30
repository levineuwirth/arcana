//! Starscream, Power Hungry // Starscream, Seeker Leader — `{3}{B}` Legendary
//! Artifact Creature — Robot 2/3 (Transform).
//! Front face: More Than Meets the Eye {2}{B}. Flying.
//! Whenever you draw a card, if you're the monarch, target opponent loses 2 life.
//! Whenever one or more creatures deal combat damage to you, convert Starscream.
//! Back face: Legendary Artifact — Vehicle. Living metal. Flying, menace, haste.
//! Whenever Starscream deals combat damage to a player, if there is no monarch,
//! that player becomes the monarch.
//! Whenever you become the monarch, convert Starscream.
//!
//! GAP: "More Than Meets the Eye" alternate casting cost not modeled.
//! GAP: "Living metal" (Vehicle is also a creature during your turn) not modeled.
//! GAP: "Convert" keyword not modeled — wired as Effect::Transform.
//! GAP: "if you're the monarch" condition on draw trigger not modeled — no monarch
//! state in the engine; trigger fires unconditionally.
//! GAP: "if there is no monarch" / "becomes the monarch" — monarch mechanic not
//! modeled; back-face-only triggered abilities not modeled.
//! GAP: back-face-only triggered abilities not modeled (abilities live on CardDefinition).
//! GAP: Vehicle crew mechanic not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Starscream, Power Hungry");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Starscream, Seeker Leader");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // P/T for Living metal (creature during your turn) — not enforced by engine
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![
                KeywordAbility::Flying,
                KeywordAbility::Menace,
                KeywordAbility::Haste,
            ],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: Whenever you draw a card, [if you're the monarch,] target opponent loses 2 life.
            // GAP: "if you're the monarch" condition not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: draw_lose_life_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_player()],
            })
            // Front: Whenever one or more creatures deal combat damage to you, convert Starscream.
            // Wired as DamageDealt with creature source filter and combat_only.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: convert_to_back,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only triggered abilities not modeled:
        // - Whenever Starscream deals combat damage to a player, if there is no monarch,
        //   that player becomes the monarch.
        // - Whenever you become the monarch, convert Starscream.
    )
}

fn draw_lose_life_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: should only apply if controller is the monarch — not checkable here.
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let arcana_core::targets::TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::LoseLife {
        player: *p,
        amount: 2,
    }]
}

fn convert_to_back(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform {
        target: trig.source,
    }]
}
