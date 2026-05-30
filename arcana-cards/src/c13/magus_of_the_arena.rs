//! Magus of the Arena — `{4}{R}{R}` 5/5 red Human Wizard.
//! `{3}, {T}: Tap target creature you control and target creature of an
//! opponent's choice they control. Those creatures fight each other.`
//! GAP: The "target creature of an opponent's choice they control" half
//! of the targeting requires the opponent to choose a target, which is
//! not expressible in the current target-requirement API (controller
//! constraint selects the activating player's creatures only). We model
//! the two targets as: [0] creature you control (you choose), [1]
//! creature an opponent controls (you choose — the opponent-choice
//! semantics are a GAP). Fight uses both ids.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Magus of the Arena");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}, {T}: Tap target creature you control and target creature of an opponent's choice they control. Those creatures fight each other.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
                TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                },
            ],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: arena_fight,
        }),
    )
}

fn arena_fight(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = &ctx.targets.targets;
    let Some(t0) = targets.first() else {
        return Vec::new();
    };
    let Some(t1) = targets.get(1) else {
        return Vec::new();
    };
    let TargetChoice::Object(id0) = t0 else {
        return Vec::new();
    };
    let TargetChoice::Object(id1) = t1 else {
        return Vec::new();
    };
    // Tap both creatures first, then fight.
    vec![
        Effect::Tap { target: *id0 },
        Effect::Tap { target: *id1 },
        Effect::Fight { a: *id0, b: *id1 },
    ]
}
