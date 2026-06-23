//! Silver-Fur Master — `{U}{B}` 2/2 Rat Ninja.
//! Ninjutsu {U}{B}.
//! Ninjutsu abilities you activate cost {1} less to activate.
//! Other Ninja and Rogue creatures you control get +1/+1.
//!
//! * Ninjutsu is not in the supported keyword surface — GAP'd.
//! * "Ninjutsu abilities you activate cost {1} less" is a cost-reduction
//!   static — GAP'd.
//! * "Other Ninja and Rogue creatures you control get +1/+1" is a static
//!   tribal anthem, wired as a `SelfEntersBattlefield` trigger that
//!   installs a `ContinuousEffect::filtered_pump` over Ninja-or-Rogue
//!   creatures you control (subtypes-any = OR). The "other" exclusion is
//!   a documented minor fidelity gap (this Rat Ninja also matches Ninja).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silver-Fur Master");
    let rat = reg.interner_mut().intern("Rat");
    let ninja = reg.interner_mut().intern("Ninja");
    // Ensure "Rogue" is interned so the anthem resolver's lookup succeeds
    // even if no Rogue card has been registered yet.
    let _rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(ninja);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: keyword — Ninjutsu is not in the supported keyword surface.
        keywords: vec![],
        ..Default::default()
    };
    // GAP: static — "Ninjutsu abilities you activate cost {1} less" is a cost-
    // reduction static, not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_ninja_rogue_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Install "Ninja and Rogue creatures you control get +1/+1" anchored to
/// this creature, lasting until it leaves the battlefield.
fn install_ninja_rogue_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let ninja = reg.interner().lookup("Ninja").unwrap_or_default();
    let rogue = reg.interner().lookup("Rogue").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![ninja, rogue]);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_pump(
            trig.source,
            filter,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
