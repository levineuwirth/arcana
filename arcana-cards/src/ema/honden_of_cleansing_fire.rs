//! Honden of Cleansing Fire — `{3}{W}` Legendary Enchantment — Shrine
//! (Champions of Kamigawa, 2004). "At the beginning of your upkeep, you
//! gain 2 life for each Shrine you control."
//!
//! Your-upkeep trigger; the amount is 2 x the number of Shrines you
//! control, counted at resolution via `script::count_matching` over a
//! Shrine subtype filter.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Honden of Cleansing Fire");
    let shrine = reg.interner_mut().intern("Shrine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shrine);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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
                effect: shrine_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you gain 2 life for each Shrine you control."
fn shrine_life(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let shrines = script::count_matching(
        state,
        &script::subtype_filter(reg, "Shrine")
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 2 * shrines,
    }]
}
