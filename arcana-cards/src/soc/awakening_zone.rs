//! Awakening Zone — `{2}{G}` enchantment (Rise of the Eldrazi, 2010).
//! "At the beginning of your upkeep, you may create a 0/1 colorless Eldrazi
//! Spawn creature token. It has 'Sacrifice this token: Add {C}.'"
//!
//! Upkeep trigger minting the token. The token's printed sacrifice-for-mana
//! activated ability is not expressible on a `TokenDefinition` (abilities
//! list takes no activated defs here) — documented GAP. The "you may" is
//! resolved as a yes.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::effects::TokenDefinition;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Awakening Zone");
    let _token_name = reg.interner_mut().intern("Eldrazi Spawn");
    let _eldrazi = reg.interner_mut().intern("Eldrazi");
    let _spawn = reg.interner_mut().intern("Spawn");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
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
                effect: spawn_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may create a 0/1 colorless Eldrazi Spawn creature token."
fn spawn_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg.interner().lookup("Eldrazi Spawn").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Eldrazi") {
        subtypes.0.insert(s);
    }
    if let Some(s) = reg.interner().lookup("Spawn") {
        subtypes.0.insert(s);
    }
    // GAP: the token's printed "Sacrifice this token: Add {C}." activated
    // ability cannot be attached to a TokenDefinition.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::new(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
