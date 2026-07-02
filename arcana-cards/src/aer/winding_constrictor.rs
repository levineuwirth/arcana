//! Winding Constrictor — `{B}{G}` 2/3 Creature — Snake.
//!
//! Oracle:
//! * If one or more counters would be put on an artifact or creature you
//!   control, that many plus one of each of those kinds are put instead.
//! * If you would get one or more counters, you get that many plus one of each
//!   of those kinds instead.
//!
//! Implementation: an ETB trigger installs a counter-placement replacement
//! (`ReplacementKind::AddAdditionalCounters(1)` on ANY counter placed on an
//! artifact/creature you control) — the Hardened Scales precedent generalized to
//! all counter kinds and to artifacts-or-creatures. The player-counter half ("if
//! you would GET counters") has no player-target replacement condition on this
//! class and stays a minor GAP (rare in practice: energy/experience aside).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    CounterKindFilter, ReplacementCondition, ReplacementDuration, ReplacementEffect,
    ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Winding Constrictor");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "counters you GET get +1" — no player-target replacement condition.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_install,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldPlaceCounters {
                object_filter: ObjectFilter::permanent()
                    .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT))
                    .controlled_by(ControllerConstraint::You),
                kinds: CounterKindFilter::Any,
            },
            kind: ReplacementKind::AddAdditionalCounters(1),
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            state_gate: None,
        }),
    }]
}
