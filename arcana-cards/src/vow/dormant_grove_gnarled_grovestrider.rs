//! Dormant Grove // Gnarled Grovestrider — `{3}{G}` transforming DFC.
//! Front (Dormant Grove — Enchantment):
//!   At the beginning of combat on your turn, put a +1/+1 counter on target
//!   creature you control. Then if that creature has toughness 6 or greater,
//!   transform Dormant Grove.
//! Back (Gnarled Grovestrider — Creature — Treefolk):
//!   Vigilance. Other creatures you control have vigilance.
//!
//! GAPs:
//! - "Then if that creature has toughness 6 or greater, transform": the
//!   conditional transform depends on the target's toughness AFTER the +1/+1
//!   counter is placed, which the resolver cannot observe (the counter effect
//!   resolves alongside this one). No post-mutation conditional primitive in
//!   the catalog; the conditional transform is not modeled. The +1/+1 counter
//!   IS placed.
//! - Back face static "Other creatures you control have vigilance" is a
//!   continuous grant-to-others static ability; not expressible on a back face.
//!   The back's own Vigilance keyword is recorded.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dormant Grove");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };

    // Back face: Gnarled Grovestrider — Creature — Treefolk, with Vigilance.
    let back_name = reg.interner_mut().intern("Gnarled Grovestrider");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(treefolk);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![KeywordAbility::Vigilance],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front (face 0): at beginning of combat on your turn, +1/+1 counter
            // on target creature you control.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: grow_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn grow_target(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
    // GAP: "then if that creature has toughness 6 or greater, transform" — the
    // post-counter conditional transform is not modeled (see file header).
}
