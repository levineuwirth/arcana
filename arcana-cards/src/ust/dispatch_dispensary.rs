//! Dispatch Dispensary — artifact — Contraption (no mana cost).
//! "Whenever you crank this Contraption, create a 2/2 black Rogue
//! creature token with menace."
//!
//! Contraptions / crank are unmodeled (the trigger condition is a
//! documented GAP); the token payoff itself is fully wired.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dispatch Dispensary");
    let contraption = reg.interner_mut().intern("Contraption");
    // Pre-intern the token subtype for the resolver's read-only lookup.
    let _rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(contraption);
    let chars = Characteristics {
        name,
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "Whenever you crank this Contraption" (Un-set
            // cranking) is not modeled; SelfBecomesTapped is the closest
            // available condition.
            trigger_condition: TriggerCondition::SelfBecomesTapped,
            intervening_if: None,
            effect: mint_rogue,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// "…create a 2/2 black Rogue creature token with menace."
fn mint_rogue(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rogue = reg.interner().lookup("Rogue").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rogue.clone());
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: rogue,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Menace],
            abilities: vec![],
        },
    }]
}
