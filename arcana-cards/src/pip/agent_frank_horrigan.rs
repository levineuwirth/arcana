//! Agent Frank Horrigan — `{5}{B}{G}` Legendary 8/6 Mutant Warrior with
//! Trample.
//! Trample
//! Agent Frank Horrigan has indestructible as long as it attacked this turn.
//! Whenever Agent Frank Horrigan enters or attacks, proliferate twice.
//!
//! Note: Scryfall lists "Proliferate" as a keyword, but Proliferate is an
//! effect (`Effect::Proliferate`), not a `KeywordAbility` variant — the
//! keyword line carries only Trample and the proliferate is wired into the
//! enters/attacks triggers.
//! GAP: the conditional static "has indestructible as long as it attacked
//! this turn" is a condition-gated keyword grant with no demonstrated static
//! primitive — omitted.
//! "enters or attacks" is decomposed into two triggers (one ETB, one
//! attacks), each proliferating twice.

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
    let name = reg.interner_mut().intern("Agent Frank Horrigan");
    let mutant = reg.interner_mut().intern("Mutant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: proliferate_twice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: proliferate_twice,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "proliferate twice" — two consecutive proliferate operations.
fn proliferate_twice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Proliferate, Effect::Proliferate]
}
