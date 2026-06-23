//! Hangar Scrounger — `{2}{R}` 2/1 Dwarf Pilot.
//!
//! Oracle:
//! Backup 1 (When this creature enters, put a +1/+1 counter on target
//!   creature. If that's another creature, it gains the following ability
//!   until end of turn.)
//! Whenever this creature becomes tapped, you may discard a card. If you
//!   do, draw a card.
//!
//! Decomposition:
//! - Backup 1 — the Backup keyword (and its targeted-counter + ability-grant
//!   rider) is not in the demonstrated KeywordAbility surface, GAP'd.
//! - "Whenever this creature becomes tapped, you may discard a card. If you
//!   do, draw a card." — wired as a becomes-tapped trigger that loots
//!   (discard one, draw one). The optional "may" is a resolution-time choice
//!   not expressible as a gate; emitted as the faithful discard-then-draw.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hangar Scrounger");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(pilot);

    // GAP (keyword): Backup 1 — not in the demonstrated KeywordAbility set;
    // its ETB +1/+1 counter on target creature plus the conditional
    // ability-grant rider has no expressible form here.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfBecomesTapped,
            intervening_if: None,
            effect: loot,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn loot(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::Discard {
            player: trig.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}
