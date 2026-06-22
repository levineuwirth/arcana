//! Worldspine Wurm — `{8}{G}{G}{G}` 15/15 green Wurm with Trample.
//!
//! * Trample.
//! * When this creature dies, create three 5/5 green Wurm creature
//!   tokens with trample.
//! * When Worldspine Wurm is put into a graveyard from anywhere, shuffle
//!   it into its owner's library. (GAP: no graveyard→library shuffle
//!   Effect — see Vigor; `Effect::Shuffle` only randomizes the library.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Worldspine Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(15)),
        toughness: Some(PtValue::Fixed(15)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_make_wurms,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: shuffle_back,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Dies: create three 5/5 green Wurm tokens with trample.
fn dies_make_wurms(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let wurm = reg.interner().lookup("Wurm").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let token = TokenDefinition {
        name: wurm,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}

/// Put into a graveyard from anywhere: shuffle it into its owner's library.
fn shuffle_back(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "shuffle it into its owner's library" from the graveyard — no
    // graveyard→library shuffle Effect variant (Effect::Shuffle only
    // randomizes the existing library). Approximated by SelfDies; other-
    // zone deaths are also unmodeled.
    Vec::new()
}
