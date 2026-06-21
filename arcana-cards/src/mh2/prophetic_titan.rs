//! Prophetic Titan — `{4}{U}{R}` 4/4 Giant Wizard.
//! Delirium — When this creature enters, choose one. If there are four or
//! more card types among cards in your graveyard, choose both instead.
//! • This creature deals 4 damage to any target.
//! • Look at the top four cards of your library. Put one of them into your
//!   hand and the rest on the bottom of your library in a random order.
//!
//! "Delirium" is an ability word (not a KeywordAbility), so no keyword is
//! emitted. The ETB is a MODAL triggered ability ("choose one … choose
//! both instead"); triggered abilities have no modal machinery in the
//! engine API, so the whole choice-bearing effect is GAP'd.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Prophetic Titan");
    let giant = reg.interner_mut().intern("Giant");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_modal(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal ("choose one … if delirium, choose both instead") ETB —
    // triggered abilities have no modal/choose-one machinery in the engine
    // API (modal is a SpellAbilityDef feature only), so neither the choice
    // between the two modes nor the delirium upgrade-to-both is expressible.
    Vec::new()
}
