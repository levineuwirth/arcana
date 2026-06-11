//! Bitterblossom — `{1}{B}` Kindred Enchantment — Faerie.
//! "At the beginning of your upkeep, you lose 1 life and create a 1/1
//! black Faerie Rogue creature token with flying."
//!
//! GAP: the Kindred card type is not modeled (`TypeLine` has no KINDRED
//! const in this API surface) — the card is registered as a plain
//! enchantment carrying the Faerie subtype.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bitterblossom");
    let faerie = reg.interner_mut().intern("Faerie");
    let _rogue = reg.interner_mut().intern("Rogue");
    let _token_name = reg.interner_mut().intern("Faerie Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: bleed_and_breed,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you lose 1 life and create a 1/1 black Faerie Rogue creature token
/// with flying."
fn bleed_and_breed(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg.interner().lookup("Faerie Rogue").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Faerie") {
        subtypes.0.insert(s);
    }
    if let Some(s) = reg.interner().lookup("Rogue") {
        subtypes.0.insert(s);
    }
    vec![
        Effect::LoseLife {
            player: trig.controller,
            amount: 1,
        },
        Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: token_name,
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![KeywordAbility::Flying],
                abilities: vec![],
            },
        },
    ]
}
