//! Weary Prisoner // Wrathful Jailbreaker — `{3}{R}` Human Werewolf 2/6 (front) with
//! Defender and Daybound. Transforms into a Werewolf (back) that attacks each combat if
//! able and has Nightbound.
//!
//! # GAPs
//! - Daybound / Nightbound (the day/night cycle and "no spells cast last turn" werewolf
//!   trigger conditions) are not modeled. The transform is wired to a begin-of-upkeep
//!   triggered ability as a best-effort approximation.
//! - Back-face-only triggered ability ("This creature attacks each combat if able") is not
//!   modeled — triggers live on the CardDefinition, not on the face.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Weary Prisoner");
    let human_sub = reg.interner_mut().intern("Human");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(werewolf_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Defender],
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Wrathful Jailbreaker");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(werewolf_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // GAP: back-face-only triggered ability "attacks each combat if able" not modeled
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(6)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Daybound front->back transform: fires if no spells were cast last turn.
            // GAP: full Daybound/Nightbound day/night state machine not modeled; gated on
            // the no-spells-last-turn condition as the closest available approximation.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(iif_no_spells),
                effect: transform_front_to_back,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn iif_no_spells(state: &GameState, _s: ObjectId, _y: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::no_spells_cast_last_turn(state)
}

fn transform_front_to_back(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
