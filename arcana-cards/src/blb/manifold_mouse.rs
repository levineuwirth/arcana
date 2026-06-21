//! Manifold Mouse — `{1}{R}` 1/2 Mouse Soldier.
//! Offspring {2}.
//! At the beginning of combat on your turn, target Mouse you control gains
//! your choice of double strike or trample until end of turn.
//!
//! Offspring is not an emittable KeywordAbility and its 1/1-token-copy ETB
//! rider is unexpressible — GAP'd.
//! The combat trigger is GAP'd: "your choice of double strike or trample" is
//! a player choice between two keyword grants, and triggered abilities have
//! no modal/choice machinery (modal is a SpellAbilityDef feature only).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Manifold Mouse");
    let mouse = reg.interner_mut().intern("Mouse");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mouse);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Offspring {2} — not an emittable KeywordAbility; token-copy rider
    // unexpressible.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: combat_grant_choice,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_grant_choice(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "your choice of double strike or trample" — player choice between
    // two keyword grants; no modal machinery for triggered abilities.
    Vec::new()
}
