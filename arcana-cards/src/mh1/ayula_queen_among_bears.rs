//! Ayula, Queen Among Bears — `{1}{G}` 2/2 Legendary Bear.
//! "Whenever another Bear you control enters, choose one —
//!  • Put two +1/+1 counters on target Bear.
//!  • Target Bear you control fights target creature you don't control."
//!
//! Scryfall lists the keyword "Fight", which is not an evergreen
//! KeywordAbility variant (it's reminder text for the modal), so the
//! keyword line is empty. The ETB-of-another-Bear trigger is wired, but
//! its choose-one modal payload is GAP'd: modal dispatch is a
//! SpellAbilityDef feature (ModalSpec / dispatch_modal_effect) and a
//! TriggeredAbilityDef has no modal slot.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ayula, Queen Among Bears");
    let bear = reg.interner_mut().intern("Bear");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);

    let bear_filter = script::subtype_filter(reg, "Bear").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: bear_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: on_bear_enters,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_bear_enters(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: choose-one modal on a triggered ability is not expressible — modal
    // dispatch (ModalSpec / dispatch_modal_effect) is a SpellAbilityDef
    // feature, and a TriggeredAbilityDef carries no modal slot. Both modes
    // (two +1/+1 counters on target Bear; Bear-you-control fights a creature)
    // also need target_requirements that the chosen mode would own.
    Vec::new()
}
