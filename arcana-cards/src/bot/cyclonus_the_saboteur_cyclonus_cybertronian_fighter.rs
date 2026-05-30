//! Cyclonus, the Saboteur // Cyclonus, Cybertronian Fighter
//!
//! Front: Legendary Artifact Creature — Robot {2}{U}{B}, 2/5
//! More Than Meets the Eye {5}{U}{B} (GAP: alternate cast cost not modeled.)
//! Flying.
//! Whenever Cyclonus deals combat damage to a player, it connives. Then if Cyclonus's power is 5
//! or greater, convert (transform) it.
//! (GAP: "Connive" not modeled — connive is draw-then-discard-and-maybe-counter; omitting.
//!  GAP: conditional transform "if power >= 5" — scripted as unconditional transform for now,
//!  will be flagged by verify.)
//!
//! Back (Cyclonus, Cybertronian Fighter): Legendary Artifact — Vehicle
//! Living metal (GAP: not modeled.)
//! Flying.
//! Whenever Cyclonus deals combat damage to a player, convert (transform) it. If you do, there is
//! an additional beginning phase after this phase. (GAP: additional phase not modeled.)
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cyclonus, the Saboteur");

    let robot = reg.interner_mut().intern("Robot");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: front_subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Cyclonus, Cybertronian Fighter");
    let vehicle = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue() | ColorSet::black(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face: Whenever Cyclonus deals combat damage to a player, connive then
            // conditionally transform.
            // GAP: Connive not modeled; transform emitted unconditionally (power >= 5 check GAP).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: front_damage_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face-only triggered ability (combat damage -> transform + additional phase)
            // not modeled.
    )
}

fn front_damage_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Connive not modeled.
    // GAP: Conditional "if power >= 5" check not modeled; emitting transform unconditionally.
    vec![Effect::Transform { target: trig.source }]
}
