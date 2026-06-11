//! The Lost and the Damned — `{1}{U}{R}` enchantment.
//! "Whenever a land you control enters from anywhere other than your
//! hand or you cast a spell from anywhere other than your hand, create a
//! 3/3 red Spawn creature token."
//!
//! Modeled as two triggered abilities (one per OR-clause), both feeding
//! the same token-mint effect. GAP on both conditions: "from anywhere
//! OTHER THAN YOUR HAND" is not expressible (`ZoneChange.from` is a
//! single `Option<Zone>`, no negation; `SpellCast` has no cast-from
//! zone), so both wired conditions over-match.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Lost and the Damned");
    let _spawn = reg.interner_mut().intern("Spawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "a land you control enters FROM ANYWHERE
                // OTHER THAN YOUR HAND" — ZoneChange.from cannot express
                // zone negation; this fires on every land entry.
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: spawn_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: trigger — "you cast a spell FROM ANYWHERE OTHER
                // THAN YOUR HAND" — SpellCast has no cast-from zone
                // field; this fires on every spell you cast.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: spawn_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "…create a 3/3 red Spawn creature token."
fn spawn_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spawn = reg.interner().lookup("Spawn").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Spawn") {
        subtypes.0.insert(s);
    }
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: spawn,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
