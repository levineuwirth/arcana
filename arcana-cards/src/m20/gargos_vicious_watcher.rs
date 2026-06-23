//! Gargos, Vicious Watcher — `{3}{G}{G}{G}` 8/7 Legendary Creature — Hydra.
//!
//! * Vigilance (keyword).
//! * "Hydra spells you cast cost {4} less to cast." — a static
//!   cost-reduction continuous ability; not expressible with the
//!   demonstrated primitives. GAP'd.
//! * "Whenever a creature you control becomes the target of a spell,
//!   Gargos fights up to one target creature you don't control." —
//!   the trigger watches OTHER creatures becoming targets; the engine's
//!   `SelfBecomesTarget` only watches this creature itself, so the
//!   trigger condition is a closest-fit GAP. The fight payload (up to
//!   one target creature you don't control) is wired faithfully via
//!   `Effect::Fight`.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gargos, Vicious Watcher");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![arcana_core::effects::KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static — "Hydra spells you cast cost {4} less to cast."

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // GAP: trigger — "a creature YOU CONTROL becomes the target of a
            // spell"; no filtered other-creature-becomes-target variant, so
            // SelfBecomesTarget (this creature only) is the closest fit.
            trigger_condition: TriggerCondition::SelfBecomesTarget {
                caster: ControllerConstraint::Any,
            },
            intervening_if: None,
            effect: gargos_fight,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                ),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn gargos_fight(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Fight {
        a: trig.source,
        b: *id,
    }]
}
