//! Alandra, Sky Dreamer — `{2}{U}{U}` 2/4 Legendary Merfolk Wizard (blue).
//! "Whenever you draw your second card each turn, create a 2/2 blue Drake
//! creature token with flying."
//! "Whenever you draw your fifth card each turn, Alandra and Drakes you
//! control each get +X/+X until end of turn, where X is the number of cards
//! in your hand."
//!
//! Decomposition:
//! * No keyword line.
//! * Trigger 1 — `TriggerCondition::CardDrawn { player: You }` gated by an
//!   intervening-if on "this is your second draw this turn"
//!   (`script::cards_drawn_this_turn == 2`); creates a Drake token.
//! * Trigger 2 — same `CardDrawn` trigger gated on the fifth draw
//!   (`== 5`); pumps Alandra and each Drake you control by +X/+X where X is
//!   your hand size. `Effect::ForEach` does NOT substitute per-id, so the
//!   pump is built as an `Effect::Sequence` over the matching object ids.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::effects::{KeywordAbility, TokenDefinition};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alandra, Sky Dreamer");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    // Pre-intern the Drake subtype so the token resolver can recover it.
    let _drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: Some(if_second_draw),
                effect: create_drake,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: Some(if_fifth_draw),
                effect: pump_alandra_and_drakes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "your second card each turn"
fn if_second_draw(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    script::cards_drawn_this_turn(s, you) == 2
}

/// "your fifth card each turn"
fn if_fifth_draw(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    script::cards_drawn_this_turn(s, you) == 5
}

/// "create a 2/2 blue Drake creature token with flying."
fn create_drake(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let drake = reg.interner().lookup("Drake").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: drake,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}

/// "Alandra and Drakes you control each get +X/+X until end of turn, where X
/// is the number of cards in your hand."
fn pump_alandra_and_drakes(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::hand_size(state, trig.controller) as i32;

    // Drakes you control.
    let drake_filter = script::subtype_filter(reg, "Drake").controlled_by(ControllerConstraint::You);
    let mut ids = script::ids_matching(state, &drake_filter, trig.controller);
    // Alandra herself (the source).
    if !ids.contains(&trig.source) {
        ids.push(trig.source);
    }

    let effects: Vec<Effect> = ids
        .into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: x,
            toughness: x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect();
    vec![Effect::Sequence(effects)]
}
