//! Wolfcaller's Howl — `{3}{G}` enchantment.
//! "At the beginning of your upkeep, create X 2/2 green Wolf creature
//! tokens, where X is the number of your opponents with four or more
//! cards in hand."
//!
//! Upkeep trigger; X is computed at resolution by counting opponents
//! whose hand size is at least four (`script::opponents` +
//! `script::hand_size`).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wolfcaller's Howl");
    // Pre-intern the token subtype for the resolver's read-only lookup.
    let _wolf = reg.interner_mut().intern("Wolf");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
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
                effect: upkeep_wolves,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create X 2/2 green Wolf creature tokens, where X is the number of
/// your opponents with four or more cards in hand."
fn upkeep_wolves(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::opponents(state, trig.controller)
        .into_iter()
        .filter(|p| script::hand_size(state, *p) >= 4)
        .count();
    let mut effects = Vec::new();
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        if let Some(s) = reg.interner().lookup("Wolf") {
            subtypes.0.insert(s);
        }
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: reg.interner().lookup("Wolf").unwrap_or_default(),
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    effects
}
