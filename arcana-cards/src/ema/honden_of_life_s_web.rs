//! Honden of Life's Web — `{4}{G}` Legendary Enchantment — Shrine.
//! "At the beginning of your upkeep, create a 1/1 colorless Spirit
//! creature token for each Shrine you control."

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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Honden of Life's Web");
    let shrine = reg.interner_mut().intern("Shrine");
    // Pre-intern the token's name/subtype for resolver lookup.
    let _spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shrine);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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
                effect: spirits_per_shrine,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 1/1 colorless Spirit creature token for each Shrine you
/// control."
fn spirits_per_shrine(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(
        state,
        &script::subtype_filter(reg, "Shrine")
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    (0..n)
        .map(|_| {
            let name = reg.interner().lookup("Spirit").unwrap_or_default();
            let mut subtypes = SubtypeSet::default();
            if let Some(s) = reg.interner().lookup("Spirit") {
                subtypes.0.insert(s);
            }
            Effect::CreateToken {
                controller: trig.controller,
                token: TokenDefinition {
                    name,
                    colors: ColorSet::new(),
                    types: TypeLine::CREATURE.into(),
                    subtypes,
                    power: Some(PtValue::Fixed(1)),
                    toughness: Some(PtValue::Fixed(1)),
                    keywords: vec![],
                    abilities: vec![],
                },
            }
        })
        .collect()
}
