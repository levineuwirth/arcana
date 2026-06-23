//! Optimus Prime, Inspiring Leader — `{3}{R}{W}` 4/5 Legendary Artifact
//! Creature — Autobot.
//!
//! Oracle:
//! * `{1}: Turn target permanent you control to its other face.` — modeled
//!   as `Effect::Transform` on the chosen permanent.
//! * `{1}: Until end of turn, Optimus Prime becomes a Construct with base
//!   power and toughness 6/6 and creatures you control gain trample.` —
//!   modeled as a base-P/T set to 6/6 on this creature plus an end-of-turn
//!   trample grant to every creature you control. (The Construct subtype
//!   change is GAP'd — `AddType` adds types, not subtypes.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Optimus Prime, Inspiring Leader");
    let autobot = reg.interner_mut().intern("Autobot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(autobot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Turn target permanent you control to its other face.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: turn_to_other_face,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Until end of turn, Optimus Prime, Inspiring Leader becomes a Construct with base power and toughness 6/6 and creatures you control gain trample.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_construct,
            }),
    )
}

fn turn_to_other_face(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Transform { target: *id }]
}

fn become_construct(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a Construct" — AddType adds card types, not creature
    // subtypes, so the Construct subtype change is omitted. The base 6/6 set
    // and the team-wide trample grant ARE expressible.
    let mut effects = vec![Effect::SetBasePT {
        target: ctx.source,
        power: 6,
        toughness: 6,
        duration: Duration::EndOfTurn,
    }];
    let creatures = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    for id in creatures {
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
