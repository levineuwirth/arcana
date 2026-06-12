//! Aang, at the Crossroads // Aang, Destined Savior
//! Front: `{2}{G}{W}{U}` Legendary Creature — Human Avatar Ally 3/3
//!   Flying
//!   ETB: Look at top 5, may put creature MV ≤ 4 onto battlefield, rest to bottom random.
//!   When another creature you control leaves the battlefield, transform Aang at beginning of next upkeep.
//!
//! Back: Legendary Creature — Avatar Ally
//!   Flying
//!   Land creatures you control have vigilance. (back-face-only static — GAP)
//!   At the beginning of combat on your turn, earthbend 2. (Earthbend not in engine — GAP)
//!
//! GAPs:
//! - ETB: "may put [creature MV ≤ 4] onto the battlefield" — RevealUntil models
//!   "reveals until found, put on battlefield" but lacks the "may" (player choice
//!   whether to put the found card on battlefield). Used as closest approximation.
//! - ZoneChange trigger: "leaves battlefield" requires specifying destination zone;
//!   approximated as going to graveyard (Zone::Graveyard(0) placeholder).
//! - Back face "land creatures you control have vigilance" — back-face-only static not modeled.
//! - Back face "earthbend 2" — Earthbend mechanic not in engine.
//! - Back face triggered abilities not modeled (GAP: back-face-only triggered ability not modeled).

use arcana_core::effects::{DelayedWhen, Effect, KeywordAbility, RevealDest, DigRest};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aang, at the Crossroads");
    let human_sub = reg.interner_mut().intern("Human");
    let avatar_sub = reg.interner_mut().intern("Avatar");
    let ally_sub = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(avatar_sub);
    subtypes.0.insert(ally_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Aang, Destined Savior");
    let avatar_back_sub = reg.interner_mut().intern("Avatar");
    let ally_back_sub = reg.interner_mut().intern("Ally");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(avatar_back_sub);
    back_subtypes.0.insert(ally_back_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(6)),
            toughness: Some(PtValue::Fixed(6)),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // ETB: look at top 5, may put creature MV ≤ 4 onto battlefield
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_look_top_five,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // When another creature you control leaves the battlefield,
            // transform Aang at the beginning of the next upkeep
            // (scheduled via ScheduleDelayedEffect).
            // GAP: "leaves battlefield" approximated as going to graveyard.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: transform_aang,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            }),
    )
}

fn etb_look_top_five(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "may put" — RevealUntil doesn't offer a "may skip" choice;
    // it deterministically places the first matching card onto the battlefield.
    vec![Effect::RevealUntil {
        player: trig.controller,
        filter: ObjectFilter::creature().with_max_cmc(4),
        found_dest: RevealDest::Battlefield,
        rest: DigRest::BottomRandom,
        max_reveal: Some(5),
    }]
}

fn transform_aang(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "transform Aang at the beginning of the next upkeep" — scheduled
    // as a delayed effect; Aang's battlefield id is stable until then.
    vec![Effect::ScheduleDelayedEffect {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextUpkeep,
        effect: delayed_transform_aang,
    }]
}

/// Delayed transform: `pt.source` is Aang's battlefield id; no-op if
/// he has left the battlefield by the upkeep.
fn delayed_transform_aang(
    _state: &GameState,
    pt: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: pt.source }]
}
