//! Krang, the All-Powerful — `{4}{U}` Legendary 3/3 Utrom Robot artifact
//! creature.
//!
//! Oracle:
//! * If a player drawing a card causes a triggered ability of a permanent
//!   you control to trigger, that ability triggers an additional time.
//!   (GAP — a static rule-modifying replacement on others' triggered
//!   abilities, with no triggered/activated wrapper to express it here.)
//! * Whenever a player draws their second card each turn, put a +1/+1
//!   counter on Krang. (Wired as a per-draw `CardDrawn` trigger gated by
//!   an intervening-if; the "second card each turn" check is approximated
//!   against the controller's draw count for this turn — the engine's iif
//!   doesn't expose the specific drawing player, so the any-player branch
//!   is a documented fidelity partial.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Krang, the All-Powerful");
    let utrom = reg.interner_mut().intern("Utrom");
    let robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(utrom);
    subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CardDrawn {
                player: ControllerConstraint::Any,
            },
            intervening_if: Some(if_second_draw_this_turn),
            effect: counter_on_krang,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn if_second_draw_this_turn(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // Approximation of "a player draws their second card each turn": gate
    // on the controller having drawn at least two cards this turn.
    script::cards_drawn_this_turn(s, you) >= 2
}

fn counter_on_krang(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
