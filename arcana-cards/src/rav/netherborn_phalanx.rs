//! Netherborn Phalanx — `{5}{B}` 2/4 black Horror.
//!
//! When this creature enters, each opponent loses 1 life for each
//! creature they control.
//! Transmute {1}{B}{B}. (GAP — Transmute is not in the usable keyword
//! surface for this class.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Netherborn Phalanx");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Transmute is not in the usable keyword surface for this class.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "When this creature enters, each opponent loses 1 life for each
            // creature they control."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: each_opponent_loses,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn each_opponent_loses(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        // Creatures that opponent controls (filter's controlled_by is
        // relative to the 3rd-arg player).
        let n = script::count_matching(
            state,
            &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            opp,
        );
        effects.push(Effect::LoseLife {
            player: opp,
            amount: n,
        });
    }
    vec![Effect::Sequence(effects)]
}
