//! Chaos Defiler — `{3}{B}{R}` 5/4 Artifact Creature — Demon Construct
//! with Trample. "Battle Cannon — When this creature enters or dies,
//! for each opponent, choose a nonland permanent that player controls.
//! Destroy one of them chosen at random." The per-opponent
//! choose-then-random-destroy effect is not expressible; the two
//! trigger conditions (enters / dies) are recorded with gapped bodies.

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
    let name = reg.interner_mut().intern("Chaos Defiler");
    let demon = reg.interner_mut().intern("Demon");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        // "Battle Cannon" is an ability word, not a keyword.
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: battle_cannon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: battle_cannon,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn battle_cannon(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "for each opponent, choose a nonland permanent that player
    // controls; destroy one of them chosen at random" — no per-opponent
    // choose-then-random-destroy effect available. Omitted.
    Vec::new()
}
