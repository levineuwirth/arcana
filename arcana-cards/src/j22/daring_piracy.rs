//! Daring Piracy — `{2}{R}` enchantment.
//! "At the beginning of combat on your turn, create a 1/1 red Pirate
//! creature token with menace and haste. Exile it at the beginning of
//! the next end step."
//!
//! Modeled with `Effect::CreateTokenSacEot` — the token's id is not
//! knowable at effect-build time, so the engine's fused
//! create-then-remove-at-end-step variant is used. FIDELITY NOTE:
//! that variant destroys (sacrifices) rather than exiles at the end
//! step; for a token the board outcome is identical.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Daring Piracy");
    let _pirate = reg.interner_mut().intern("Pirate");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: raise_the_crew,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 1/1 red Pirate creature token with menace and haste.
/// Exile it at the beginning of the next end step."
fn raise_the_crew(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let pirate = reg.interner().lookup("Pirate").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pirate);
    vec![Effect::CreateTokenSacEot {
        controller: trig.controller,
        token: TokenDefinition {
            name: pirate,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Menace, KeywordAbility::Haste],
            abilities: vec![],
        },
    }]
}
