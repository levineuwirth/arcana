//! Brimaz, King of Oreskos — `{1}{W}{W}` 3/4 Legendary Cat Soldier with
//! Vigilance. Attack/block triggers each mint a 1/1 white Cat Soldier
//! token with vigilance — attacking on the attack trigger, "blocking
//! that creature" on the block trigger (the blocking placement is a GAP
//! since there's no create-token-blocking effect; a plain token is made).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Brimaz, King of Oreskos");
    let cat = reg.interner_mut().intern("Cat");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
            // "Whenever Brimaz attacks, create a 1/1 white Cat Soldier
            // creature token with vigilance that's attacking."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_cat_attacking,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "Whenever Brimaz blocks a creature, create a 1/1 white Cat
            // Soldier creature token with vigilance that's blocking that
            // creature."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: make_cat_blocking,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Build the 1/1 white Cat Soldier token with vigilance.
fn cat_soldier_token(reg: &CardRegistry) -> TokenDefinition {
    let cat = reg.interner().lookup("Cat").unwrap_or_default();
    let soldier = reg.interner().lookup("Soldier").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(soldier);
    TokenDefinition {
        name: cat,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        abilities: vec![],
    }
}

fn make_cat_attacking(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateTokenTappedAttacking {
        controller: trig.controller,
        token: cat_soldier_token(reg),
    }]
}

/// GAP: "that's blocking that creature" — there is no create-token-
/// blocking effect; we mint a plain token instead of placing it as a
/// blocker of the attacker Brimaz blocked.
fn make_cat_blocking(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: cat_soldier_token(reg),
    }]
}
