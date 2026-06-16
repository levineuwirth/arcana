//! Lord of the Void — `{4}{B}{B}{B}` 7/7 Demon with Flying.
//!
//! Oracle:
//! * Flying.
//! * Whenever this creature deals combat damage to a player, exile the
//!   top seven cards of that player's library, then put a creature card
//!   from among them onto the battlefield under your control.
//!
//! Flying is a base characteristic. The combat-damage trigger's effect
//! (exile N from the damaged player's library, then put a creature from
//! among them onto the battlefield under YOUR control) has no
//! expressible primitive — RevealUntil / DigTopN operate on the
//! controller's own library and cannot place a card under a different
//! controller — so the effect is GAP'd while the trigger shape is kept.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lord of the Void");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: exile_and_steal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn exile_and_steal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile top seven of the damaged player's library, then put a
    // creature from among them onto the battlefield under your control —
    // no primitive operates on another player's library + cross-control
    // put.
    Vec::new()
}
