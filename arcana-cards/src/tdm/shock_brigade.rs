//! Shock Brigade — `{1}{R}` 1/3 red Goblin Soldier with Menace and Mobilize 1.
//!
//! "Menace
//!  Mobilize 1 (Whenever this creature attacks, create a tapped and attacking
//!  1/1 red Warrior creature token. Sacrifice it at the beginning of the next
//!  end step.)"
//!
//! Menace is a base keyword. Mobilize 1 is the attack trigger: it mints a
//! tapped-and-attacking 1/1 red Warrior token via
//! `Effect::CreateTokenTappedAttacking`.
//!
//! FIDELITY GAP: the "sacrifice it at the beginning of the next end step" rider
//! is not attached — `CreateTokenTappedAttacking` mints the tapped/attacking
//! token but has no end-step sacrifice; the token persists.

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
    let name = reg.interner_mut().intern("Shock Brigade");
    let goblin = reg.interner_mut().intern("Goblin");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: mobilize_one,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn mobilize_one(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(warrior);
    vec![Effect::CreateTokenTappedAttacking {
        controller: trig.controller,
        token: TokenDefinition {
            name: warrior,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
