//! Witty Demon — `{1}{B}{B}` 4/3 Demon.
//! Flying. "When Witty Demon enters, if your starting library had at least
//!  13 cards over the minimum, search your library for a card and put it
//!  into your hand, then shuffle. Otherwise, Witty Demon deals 4 damage to
//!  you."
//!
//! Flying is a base keyword. The ETB ability branches on a deck-construction
//! property ("starting library ≥ 13 over the minimum") that no condition
//! helper can evaluate, so the branch selection — and therefore the whole
//! effect — is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Witty Demon");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: on_enter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_enter(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the branch depends on "your starting library had at least 13 cards
    // over the minimum" — a deck-construction property no condition helper can
    // evaluate. Without resolving the branch, neither the tutor nor the 4-damage
    // half can be emitted faithfully.
    Vec::new()
}
