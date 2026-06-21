//! Cult of Skaro — `{1}{U}{B}{R}` 4/4 Legendary Artifact Creature — Dalek.
//!
//! Whenever Cult of Skaro attacks, choose one at random —
//! • Thay — Put a +1/+1 counter on each artifact creature you control.
//! • Caan — Draw two cards.
//! • Sec — Create a 3/3 black Dalek artifact creature token with menace.
//! • Jast — Each opponent loses 4 life.
//!
//! The bespoke "Sec/Caan/Jast/Thay" entries are the named modes of this
//! attack trigger, not standalone keywords (no `KeywordAbility` variant
//! exists for them) — so `keywords` is empty.
//!
//! GAP: the "choose one at RANDOM" attack trigger has no random-modal
//! primitive on triggered abilities (modal dispatch is spell-only). The
//! whole effect is left unexpressed rather than firing one fixed mode.

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
    let name = reg.interner_mut().intern("Cult of Skaro");
    let dalek = reg.interner_mut().intern("Dalek");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dalek);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: choose_one_at_random,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn choose_one_at_random(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "choose one at random" on an attack trigger — no random-modal
    // primitive exists for triggered abilities (modal dispatch is
    // spell-only), so none of the four named modes is emitted.
    Vec::new()
}
