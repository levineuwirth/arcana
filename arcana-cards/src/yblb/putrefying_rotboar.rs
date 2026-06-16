//! Putrefying Rotboar — `{2}{B}{B}` 4/5 Elemental Boar with Lifelink.
//!
//! * Lifelink (keyword).
//! * Whenever a Boar you control attacks, each nonland card in defending
//!   player's hand perpetually gains "When you cast this spell, you lose 1
//!   life." — GAP: perpetual ability-granting to cards in hand is not
//!   expressible. The trigger is wired but its effect is a no-op.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Putrefying Rotboar");
    let elemental = reg.interner_mut().intern("Elemental");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(boar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    let boar_filter = script::subtype_filter(reg, "Boar").controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CreatureAttacks { filter: boar_filter },
            intervening_if: None,
            effect: perpetual_lose_life,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn perpetual_lose_life(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: perpetually granting "When you cast this spell, you lose 1 life." to
    // each nonland card in the defending player's hand is not expressible.
    Vec::new()
}
