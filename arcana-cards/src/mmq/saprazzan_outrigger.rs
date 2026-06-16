//! Saprazzan Outrigger — `{3}{U}` 5/5 Merfolk.
//! "When this creature attacks or blocks, put it on top of its
//! owner's library at end of combat."
//!
//! Note: "attacks or blocks" requires two triggers; only one
//! TriggeredAbilityDef is supported. Using SelfAttacks as primary;
//! adding a GAP note for the blocks trigger.
//! "at end of combat" — DelayedAction with NextEndStep is the closest
//! available; no EndOfCombat DelayedWhen variant exists.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saprazzan Outrigger");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "attacks or blocks" — only one trigger registered;
                // SelfBlocks trigger not included.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "at end of combat" — no EndOfCombat DelayedWhen variant;
    // using PutOnTopOfLibrary directly (fires at end of step not
    // end of combat).
    vec![Effect::PutOnTopOfLibrary { target: trig.source }]
}
