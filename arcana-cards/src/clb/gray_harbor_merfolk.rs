//! Gray Harbor Merfolk — `{1}{U}` 0/3 Merfolk Rogue.
//!
//! This creature can't be blocked.
//! This creature gets +2/+0 as long as you control a commander that's
//!   a creature or planeswalker.
//!
//! The "can't be blocked" static is wired via the continuous-effect
//! idiom: a `SelfEntersBattlefield` trigger installs a
//! `ContinuousEffect::cant_be_blocked` targeting this creature for
//! `Duration::WhileSourceOnBattlefield` (auto-expires when it leaves).
//! The conditional +2/+0 line is gated on controlling a commander
//! that's a creature or planeswalker — a turn-independent but
//! commander-zone-aware conditional pump with no expressible
//! constructor — so it stays GAP'd.

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
    let name = reg.interner_mut().intern("Gray Harbor Merfolk");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static "+2/+0 as long as you control a commander that's a
    // creature or planeswalker." — commander-zone-conditional pump,
    // no expressible conditional-static constructor.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cant_be_blocked,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB: install the always-on "this creature can't be blocked" static,
/// anchored to this creature, lasting while it's on the battlefield.
fn install_cant_be_blocked(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::cant_be_blocked(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
