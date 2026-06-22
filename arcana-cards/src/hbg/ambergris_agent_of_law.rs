//! Ambergris, Agent of Law — `{2}{R}{W}` 4/3 Legendary Dwarf Cleric.
//!
//! * Haste.
//! * Whenever Ambergris attacks, you may discard your hand and draw two cards.
//!   If you do, other creatures you control get +X/+X until end of turn, where
//!   X is the number of cards you've discarded this turn. → attack trigger:
//!   discard the whole hand, draw two, then pump each OTHER creature you
//!   control by +X/+X where X = cards discarded this turn (including the hand
//!   just discarded).
//!   // GAP: the "you may" optionality is not modeled (applied
//!   unconditionally). The effect vec is computed before it executes, so X is
//!   derived as (prior discards this turn) + (hand size being discarded) to
//!   reflect the post-discard count — a documented ordering fidelity gap.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ambergris, Agent of Law");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_wheel_and_pump,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_wheel_and_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may" optionality not modeled — applied unconditionally.
    let hand = script::hand_size(state, trig.controller);
    // Post-discard count for "cards you've discarded this turn".
    let x = script::cards_discarded_this_turn(state, trig.controller) + hand;

    let mut effects = vec![
        Effect::Discard {
            player: trig.controller,
            count: hand,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 2,
        },
    ];

    if x > 0 {
        // Other creatures you control — exclude Ambergris itself.
        let ids: Vec<_> = script::ids_matching(
            state,
            &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            trig.controller,
        )
        .into_iter()
        .filter(|id| *id != trig.source)
        .collect();
        if !ids.is_empty() {
            effects.push(Effect::ForEach {
                targets: ids,
                effect: Box::new(Effect::Pump {
                    target: NULL_OBJECT_ID,
                    power: x as i32,
                    toughness: x as i32,
                    duration: Duration::EndOfTurn,
                    keywords: vec![],
                }),
            });
        }
    }

    effects
}
