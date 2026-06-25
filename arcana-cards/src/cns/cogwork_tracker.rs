//! Cogwork Tracker — `{4}` 4/4 Artifact Creature — Dog Construct.
//! - "Reveal this card as you draft it…" — GAP: a draft-matters static, not a
//!   game-zone ability.
//! - "This creature attacks each combat if able." — SELF must-attack
//!   (CR 508.1a), installed on ETB.
//! - "…attacks a player you noted for cards named Cogwork Tracker each combat
//!   if able." — GAP: draft-noted-player attack-direction restriction, not
//!   modeled.

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
    let name = reg.interner_mut().intern("Cogwork Tracker");
    let dog = reg.interner_mut().intern("Dog");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(construct);

    // "attacks each combat if able" is wired as the SELF must-attack below.
    // GAP: draft-reveal / noted-player attack-direction restriction — draft
    // mechanic, not modeled.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: install_must_attack,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn install_must_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
