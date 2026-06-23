//! Bearer of Silence — `{1}{B}` 2/1 Eldrazi with Flying.
//!
//! Devoid (the card is colorless).
//! "When you cast this spell, you may pay {1}{C}. If you do, target opponent
//!  sacrifices a creature." — a cast trigger with an optional payment; there is
//!  no matching trigger condition for "when you cast this spell" (GAP).
//! Flying — wired as a base keyword.
//! "This creature can't block." — WIRED as an ETB-installed self-restriction
//!  `ContinuousEffect::cant_block(source, source, …)`, lasting while Bearer of
//!  Silence is on the battlefield (the self-restriction idiom: target =
//!  trig.source).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Bearer of Silence");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "When you cast this spell, you may pay {1}{C}. If you do, target
    //      opponent sacrifices a creature." — a cast trigger with an optional
    //      payment; no matching trigger condition for "when you cast this spell".
    reg.register(
        CardDefinition::new(name, chars)
            // "This creature can't block." installed on ETB as a self-restriction.
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

/// Install "this creature can't block" anchored to Bearer of Silence itself,
/// lasting while it remains on the battlefield.
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
