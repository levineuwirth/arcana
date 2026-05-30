//! Dumb Ass — `{2}{R}` 3½/2 red Donkey Barbarian. "At the beginning of
//! your upkeep, flip a coin. If you lose the flip, target opponent
//! chooses whether this creature attacks this turn."
//!
//! GAP: Power is 3½ (fractional); `PtValue::Fixed` only supports `i32`.
//! Emitted as 3 (truncated). The half-power is an Un-set mechanic.
//! GAP: "target opponent chooses whether this creature attacks this turn"
//! is not expressible — no Effect variant for forcing/forbidding attack
//! based on opponent's choice. The coin-flip fires; the lose branch
//! returns Vec::new() (no-op) as a stub.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dumb Ass");
    let donkey = reg.interner_mut().intern("Donkey");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(donkey);
    subtypes.0.insert(barbarian);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: Power is 3½; represented as 3 (PtValue::Fixed only supports i32)
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_flip,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_flip(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: lose branch should let target opponent choose whether this
    // creature attacks this turn — no Effect variant for this interaction.
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::Sequence(vec![])),
        lose: None,
    }]
}
