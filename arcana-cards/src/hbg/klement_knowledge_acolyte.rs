//! Klement, Knowledge Acolyte — `{1}{W}{U}` 3/4 Legendary Creature —
//! Tiefling Cleric (white-blue).
//!
//! Vigilance. When this creature specializes, seek three nonland permanent
//! cards; choose one of those cards and shuffle the rest into your library.
//!
//! Vigilance is a base keyword. The specialize trigger uses
//! `TriggerCondition::SelfSpecializes`; its effect ("seek three nonland
//! permanent cards, choose one, shuffle the rest in") is a hidden library
//! search with no `Effect` variant, so the trigger body is GAP'd while the
//! trigger itself is wired.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
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
    let name = reg.interner_mut().intern("Klement, Knowledge Acolyte");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: on_specialize_seek_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_specialize_seek_three(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek three nonland permanent cards, choose one, shuffle the rest
    // into your library" is a hidden library search with no Effect variant.
    Vec::new()
}
