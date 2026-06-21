//! Titan of Industry — `{4}{G}{G}{G}` 7/7 Elemental with Reach and Trample.
//! "Reach, trample.
//!  When this creature enters, choose two —
//!   • Destroy target artifact or enchantment.
//!   • Target player gains 5 life.
//!   • Create a 4/4 green Rhino Warrior creature token.
//!   • Put a shield counter on a creature you control."
//!
//! Reach + Trample are base keywords. The ETB trigger condition is recorded
//! as `SelfEntersBattlefield`; its payload is GAP'd:
//! * GAP: the "choose two —" MODAL payload is not expressible on a triggered
//!   ability — `ModalSpec`/`dispatch_modal_effect` are SpellAbilityDef-only,
//!   and the modes carry their own targets (artifact/enchantment, a player, a
//!   creature) which a flat trigger effect fn cannot post or choose among.

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
    let name = reg.interner_mut().intern("Titan of Industry");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_choose_two,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_choose_two(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "choose two —" modal payload not expressible on a triggered
    // ability (modal machinery is SpellAbilityDef-only; modes declare their
    // own targets a flat trigger effect fn cannot post or select among).
    Vec::new()
}
