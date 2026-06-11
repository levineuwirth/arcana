//! Deeproot Waters — `{2}{U}` enchantment (Ixalan, 2017).
//! "Whenever you cast a Merfolk spell, create a 1/1 blue Merfolk creature
//! token with hexproof."
//!
//! A subtype-filtered `SpellCast` trigger (Merfolk spells you cast) minting
//! a 1/1 blue Merfolk token with hexproof.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
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
    let name = reg.interner_mut().intern("Deeproot Waters");
    let _merfolk = reg.interner_mut().intern("Merfolk");
    let merfolk_filter = script::subtype_filter(reg, "Merfolk");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(merfolk_filter),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: school_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 1/1 blue Merfolk creature token with hexproof."
fn school_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let merfolk = reg.interner().lookup("Merfolk").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Merfolk") {
        subtypes.0.insert(s);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: merfolk,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Hexproof],
            abilities: vec![],
        },
    }]
}
