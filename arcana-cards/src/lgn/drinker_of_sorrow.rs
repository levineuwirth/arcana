//! Drinker of Sorrow — `{2}{B}` 5/3 Horror.
//!
//! Oracle:
//! * This creature can't block. — wired as a self-static: a
//!   SelfEntersBattlefield trigger installs a `CantBlock` continuous effect
//!   on this creature for as long as it remains on the battlefield.
//! * Whenever this creature deals combat damage, sacrifice a permanent.
//!
//! The combat-damage trigger is GAP'd: `TriggerCondition::DamageDealt`
//! matches its source by filter, not by the trigger's own id, so it cannot
//! be scoped to "this creature deals combat damage" without firing for every
//! creature. A faithful 5/3 Horror body with its can't-block static is
//! registered.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drinker of Sorrow");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Whenever this creature deals combat damage, sacrifice a
    //       permanent." — DamageDealt cannot be scoped to the source's own id.
    reg.register(
        CardDefinition::new(name, chars)
            // "This creature can't block." — install a self-scoped CantBlock
            // continuous effect on ETB, lasting while this creature stays on
            // the battlefield.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cant_block,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install "this creature can't block" on itself for as long as it
/// remains on the battlefield.
fn install_cant_block(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::cant_block(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
