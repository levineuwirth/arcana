//! Mangled Soulrager — `{1}{U}{R}` 1/4 Spirit with Flying and Cycling {1}{U}.
//!
//! Oracle:
//!  * Flying.
//!  * When this enters, switch the power and toughness of each creature on the
//!    battlefield. You get a twelve-time boon with "Whenever a creature you
//!    control enters, switch its power and toughness."
//!  * Cycling {1}{U}.
//!
//! Flying and Cycling are wired. The ETB (and the boon it grants) rely on a
//! "switch power and toughness" effect that has no Effect variant — GAP'd.

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
    let name = reg.interner_mut().intern("Mangled Soulrager");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Cycling(ManaCost::parse("{1}{U}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_switch_pt,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_switch_pt(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "switch the power and toughness of each creature on the battlefield"
    // and the granted twelve-time boon ("whenever a creature you control
    // enters, switch its power and toughness") both require a switch-P/T effect
    // that has no Effect variant.
    Vec::new()
}
