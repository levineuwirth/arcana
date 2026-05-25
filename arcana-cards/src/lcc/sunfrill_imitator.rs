//! Sunfrill Imitator — `{2}{G}` 3/3 green Dinosaur. "Whenever this
//! creature attacks, you may have it become a copy of another target
//! Dinosaur you control, except its name is Sunfrill Imitator and it
//! has this ability." Best-effort: triggers on attack and copies the
//! target Dinosaur via `Effect::CopyPermanent`. The "you may"
//! optionality, the "another" (exclude self) refinement, and the
//! "except its name is ~ and it has this ability" copy rider are not
//! expressible in the current engine — see GAPs.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
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
    let name = reg.interner_mut().intern("Sunfrill Imitator");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack_copy_dinosaur,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            // GAP: "you may" — the engine has no optional-trigger modifier;
            // this trigger always fires and resolves if a legal target exists.
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    script::subtype_filter(reg, "Dinosaur")
                        .controlled_by(ControllerConstraint::You),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn on_attack_copy_dinosaur(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // GAP: "except its name is Sunfrill Imitator and it has this ability" —
    // `Effect::CopyPermanent` has no "except"-clause rider, so the copy
    // will lose its identity-preserving name and the attack trigger.
    // GAP: "another" — no way to exclude `trig.source` from the target set;
    // the engine's ObjectFilter has no "other than source" refinement.
    vec![Effect::CopyPermanent { target: *id }]
}
