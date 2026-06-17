//! Klement, Life Acolyte — `{1}{W}{W}` 3/3 Legendary Tiefling Cleric with Lifelink.
//! "When this creature specializes, you get a one-time boon with 'When you cast
//!  a creature spell, that creature enters with a lifelink counter on it.'"
//!
//! Lifelink is expressible. The specialize trigger fires via SelfSpecializes,
//! but the "one-time boon" payoff (a persistent player-emblem-style triggered
//! ability granting lifelink counters to cast creatures) has no Effect form
//! here, so the trigger body is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Klement, Life Acolyte");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: specialize_boon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn specialize_boon(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you get a one-time boon with '<triggered ability>'" — a persistent
    // player-scoped boon granting lifelink counters is not modeled.
    Vec::new()
}
