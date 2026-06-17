//! Chaos Balor — `{3}{R}{R}` 4/5 Demon with Flying.
//!
//! * Flying.
//! * "Whenever Chaos Balor attacks or dies, choose two. Each mode must target a
//!   different player. • discard-and-seek • 2 damage + two Treasures • 2 damage to each
//!   creature a player controls + perpetual +2/+0."
//!   GAP: modal "choose two" is only expressible on SPELL abilities (SpellAbilityDef.
//!   modal), not on triggered abilities. The Seek mechanic and "perpetually get +2/+0"
//!   are also not modeled. We record the two trigger occurrences (attacks / dies) with
//!   GAP'd effects rather than fabricate a triggered-modal dispatcher.
//! * Seek, Treasure keywords are not usable KeywordAbility variants.

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
    let name = reg.interner_mut().intern("Chaos Balor");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: chaos_balor_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: chaos_balor_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn chaos_balor_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose two" modal effect is not expressible on a triggered ability
    // (modal dispatch exists only for SpellAbilityDef); Seek and perpetual +2/+0 are
    // also unmodeled.
    Vec::new()
}
