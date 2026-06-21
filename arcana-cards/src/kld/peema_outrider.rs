//! Peema Outrider — `{2}{G}{G}` 3/3 Creature — Elf Artificer.
//!
//! Oracle:
//! * "Trample" — keyword.
//! * "Fabricate 1 (When this creature enters, put a +1/+1 counter on it
//!   or create a 1/1 colorless Servo artifact creature token.)" —
//!   Fabricate is NOT in the usable KeywordAbility surface. Its reminder
//!   text is an ETB with a player CHOICE between two effects; triggered
//!   abilities have no modal/choose-one mechanism in the demonstrated
//!   API, so the ETB body is GAP'd.

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
    let name = reg.interner_mut().intern("Peema Outrider");
    let elf = reg.interner_mut().intern("Elf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: fabricate_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn fabricate_one(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Fabricate 1 — ETB modal choice ("+1/+1 counter on it" OR
    // "create a 1/1 Servo token") is not expressible for triggered
    // abilities (no choose-one mechanism).
    Vec::new()
}
