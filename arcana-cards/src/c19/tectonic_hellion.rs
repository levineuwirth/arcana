//! Tectonic Hellion — `{5}{R}{R}` 8/5 Hellion.
//! Haste.
//! Whenever this creature attacks, each player who controls the most lands
//! sacrifices two lands of their choice.
//!
//! Haste is a base keyword. The attack trigger is GAP'd: identifying "each
//! player who controls the most lands" requires a cross-player land-count
//! comparison that the exposed script helpers can't express, and there is
//! no per-player Sacrifice that targets only the land-leaders.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Tectonic Hellion");
    let hellion = reg.interner_mut().intern("Hellion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hellion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_sac_lands,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_sac_lands(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player who controls the most lands sacrifices two lands" —
    // no cross-player land-count comparison to select the leaders, and no
    // per-player filtered sacrifice keyed on that comparison.
    Vec::new()
}
