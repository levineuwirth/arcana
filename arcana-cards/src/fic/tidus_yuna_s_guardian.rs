//! Tidus, Yuna's Guardian — `{G}{W}{U}` 3/3 Legendary Human Warrior.
//!
//! * "At the beginning of combat on your turn, you may move a counter from
//!   target creature you control onto a second target creature you control."
//!   The trigger (combat on your turn) is expressible, but there is no
//!   demonstrated primitive to MOVE a counter between two chosen targets —
//!   `Effect::Proliferate` is the only counter-mover shown and it does not
//!   relocate counters. Effect GAP'd (returns `Vec::new()`); the trigger is
//!   still wired so the catalog records it.
//! * "Cheer — Whenever one or more creatures you control with counters on
//!   them deal combat damage to a player, you may draw a card and
//!   proliferate. Do this only once each turn." `Cheer` is not a
//!   demonstrated `KeywordAbility`; the trigger condition ("creatures you
//!   control WITH COUNTERS deal combat damage") cannot be expressed by any
//!   demonstrated `TriggerCondition` (no has-counters source filter on the
//!   combat-damage trigger). GAP'd entirely.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tidus, Yuna's Guardian");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keywords Proliferate / Cheer are not demonstrated
        // KeywordAbility variants (Proliferate is an Effect, not a keyword).
        ..Default::default()
    };

    // GAP: "Cheer — Whenever one or more creatures you control with counters
    // on them deal combat damage to a player, you may draw a card and
    // proliferate." No TriggerCondition expresses "source has counters", so
    // the whole triggered ability is omitted.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: move_counter_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn move_counter_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no demonstrated primitive moves a counter from one chosen target
    // creature onto a second chosen target creature.
    Vec::new()
}
