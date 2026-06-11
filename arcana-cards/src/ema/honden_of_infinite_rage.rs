//! Honden of Infinite Rage — `{2}{R}` legendary enchantment — Shrine
//! (Champions of Kamigawa, 2004). "At the beginning of your upkeep,
//! Honden of Infinite Rage deals damage to any target equal to the
//! number of Shrines you control."
//!
//! An upkeep trigger with an any-target requirement; the amount is the
//! live Shrine count read at resolution via `script::count_matching`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Honden of Infinite Rage");
    let shrine = reg.interner_mut().intern("Shrine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shrine);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                effect: shrine_burn,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            },
        ),
    )
}

/// "…deals damage to any target equal to the number of Shrines you
/// control."
fn shrine_burn(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let target = match trig.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(p)) => DamageTarget::Player(*p),
        Some(TargetChoice::ObjectOrPlayer(op)) => match op {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
        _ => return Vec::new(),
    };
    let filter = script::subtype_filter(reg, "Shrine")
        .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, trig.controller);
    vec![Effect::DealDamage {
        target,
        amount: n,
        source: trig.source,
    }]
}
