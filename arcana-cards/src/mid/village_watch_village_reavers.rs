//! Village Watch // Village Reavers — `{4}{R}` Human Werewolf creature 4/3.
//! Front: Haste. Daybound.
//! Back: Wolves and Werewolves you control have haste. Nightbound.
//!
//! GAP: Daybound/Nightbound keywords / day-night designator are not in the engine
//! keyword surface. The werewolf transform itself is modeled via the symmetric
//! upkeep triggers (face-gated 0/1). The back face's "Wolves and Werewolves you
//! control have haste" is a filtered keyword anthem installed once from the ETB
//! trigger with Duration::WhileSourceShowsFace(1) — dimmed on the front face, live
//! on Village Reavers.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Village Watch");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    // Pre-intern "Wolf" for the back-face anthem filter lookup.
    let _ = reg.interner_mut().intern("Wolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Haste],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Village Reavers");
    let mut back_subtypes = SubtypeSet::default();
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    back_subtypes.0.insert(back_werewolf_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // "Wolves and Werewolves you control have haste" — installed from the
            // ETB trigger with Duration::WhileSourceShowsFace(1) below.
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back-face static "Wolves and Werewolves you control have haste",
            // installed once from the ETB trigger; the face-gated duration keeps
            // it dimmed while the front (Village Watch) shows.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_back_face_haste_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Daybound: front->back transform if no spells were cast last turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_no_spells_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Nightbound: back->front transform if a player cast two or more spells last turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_player_cast_two_last_turn),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 1 (anthem install) fires on ETB regardless of face — the
            // face-gated duration handles liveness. Day-flip (2) front only;
            // night-flip (3) back only.
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 1),
    )
}

fn install_back_face_haste_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Wolves and Werewolves you control have haste" — live only while the back
    // face (Village Reavers, visible_face == 1) shows.
    let subs: Vec<_> = ["Wolf", "Werewolf"]
        .iter()
        .filter_map(|n| reg.interner().lookup(n))
        .collect();
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_keyword(
            trig.source,
            ObjectFilter::creature()
                .with_subtypes_any(subs)
                .controlled_by(ControllerConstraint::You),
            KeywordAbility::Haste,
            Duration::WhileSourceShowsFace(1),
        ),
    }]
}

fn transform_self(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}

fn if_no_spells_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(s)
}

fn if_player_cast_two_last_turn(s: &GameState, _src: ObjectId, _you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::a_player_cast_two_or_more_last_turn(s)
}
