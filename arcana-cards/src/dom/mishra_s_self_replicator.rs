//! Mishra's Self-Replicator — `{5}` 2/2 colorless Artifact Creature — Assembly-Worker.
//! "Whenever you cast a historic spell, you may pay {1}. If you do, create a token that's
//! a copy of this creature. (Artifacts, legendaries, and Sagas are historic.)"
//!
//! # Notes
//! GAP: "historic spell" — no historic spell filter in ObjectFilter. Using SpellCast with
//! artifact type as partial approximation (misses legendaries and Sagas).
//! GAP: optional pay-{1} cost gate — not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mishra's Self-Replicator");
    let assembly_worker = reg.interner_mut().intern("Assembly-Worker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(assembly_worker);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: "historic spell" — using artifact spell as partial approximation.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter {
                        types_any: Some(TypeLine::ARTIFACT.into()),
                        ..Default::default()
                    }),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: historic_copy_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn historic_copy_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: optional pay-{1} cost gate — not expressible.
    vec![Effect::CopyPermanent { target: trig.source }]
}
