//! Stensia Uprising — `{2}{R}{R}` enchantment.
//! "At the beginning of your end step, create a 1/1 red Human creature
//! token. Then if you control exactly thirteen permanents, you may
//! sacrifice this enchantment. When you do, it deals 7 damage to any
//! target."
//!
//! The end-step token mint is wired; the exactly-thirteen sacrifice
//! clause with its reflexive damage trigger is a documented GAP.

use arcana_core::effects::{Effect, TokenDefinition};
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
    let name = reg.interner_mut().intern("Stensia Uprising");
    let _human = reg.interner_mut().intern("Human");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: raise_the_mob,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 1/1 red Human creature token."
fn raise_the_mob(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human = reg.interner().lookup("Human").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    if let Some(s) = reg.interner().lookup("Human") {
        subtypes.0.insert(s);
    }
    // GAP: "Then if you control exactly thirteen permanents, you may
    // sacrifice this enchantment. When you do, it deals 7 damage to any
    // target." — the exactly-N condition, the cost-free may-sacrifice,
    // and the reflexive when-you-do targeted trigger are not expressible;
    // only the token mint is emitted.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: human,
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
