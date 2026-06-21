//! Leela, Sevateem Warrior — `{3}{G}` 3/3 Legendary Creature —
//! Human Warrior.
//! "Whenever an opponent draws a card except the first one they draw in
//!   each of their draw steps, put a +1/+1 counter on Leela."
//! "Doctor's companion (You can have two commanders if the other is the
//!   Doctor.)"
//!
//! Doctor's companion is not a usable `KeywordAbility` and is a
//! deck-construction note, not a gameplay ability — GAP'd. The
//! opponent-draw trigger fires on each opponent card draw; the "except the
//! first in each of their draw steps" exception is not expressible and is
//! a documented fidelity GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leela, Sevateem Warrior");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
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
                player: ControllerConstraint::Opponent,
            },
            intervening_if: None,
            effect: counter_on_leela,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn counter_on_leela(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Fidelity GAP: the "except the first one they draw in each of their
    // draw steps" exclusion is not expressible; the trigger fires on each
    // opponent draw.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
