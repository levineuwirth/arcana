//! Heroes in a Half Shell — `{W}{U}{B}{R}{G}` Legendary 5/5 Creature —
//! Mutant Ninja Turtle.
//! "Vigilance, menace, trample, haste"
//! "Whenever one or more Mutants, Ninjas, and/or Turtles you control deal
//!  combat damage to a player, put a +1/+1 counter on each of those creatures
//!  and draw a card."
//!
//! Decomposition:
//! - Keyword line: Vigilance, Menace, Trample, Haste.
//! - "Whenever one or more Mutants/Ninjas/Turtles you control deal combat
//!   damage to a player, …" → a DamageDealt(combat, to a player) triggered
//!   ability filtered to those subtypes.
//!   Effect: draw a card (expressible).
//!   GAP: "put a +1/+1 counter on each of those creatures" — the engine cannot
//!   identify the exact set of creatures that dealt combat damage this combat,
//!   so the counter clause is omitted.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heroes in a Half Shell");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    let source_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![mutant, ninja, turtle]);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Menace,
            KeywordAbility::Trample,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter,
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: draw_card,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// Draw a card. GAP: "put a +1/+1 counter on each of those creatures" is not
/// expressible (the exact damaging set can't be recovered).
fn draw_card(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
