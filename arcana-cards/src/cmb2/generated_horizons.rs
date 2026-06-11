//! Generated Horizons — `{2}{G}{G}` enchantment (Kaladesh Remastered /
//! Commander 2021 pool). "At the beginning of your upkeep, create a
//! Forest land token."
//!
//! An upkeep trigger minting a colorless Forest land token (no
//! power/toughness; the Forest subtype carries the intrinsic
//! {T}: Add {G}).

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
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Generated Horizons");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
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
                effect: grow_forest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a Forest land token."
fn grow_forest(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg.interner().lookup("Forest").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(forest);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: forest,
            colors: ColorSet::new(),
            types: TypeLine::LAND.into(),
            subtypes,
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
