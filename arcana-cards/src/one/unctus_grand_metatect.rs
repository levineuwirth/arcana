//! Unctus, Grand Metatect — `{1}{U}{U}` 2/4 Legendary Artifact Creature
//! — Phyrexian Vedalken (U).
//!
//! Oracle:
//! * Other blue creatures you control have "Whenever this creature
//!   becomes tapped, draw a card, then discard a card." (static
//!   ability-grant — GAP).
//! * Other artifact creatures you control get +1/+1. (static anthem —
//!   GAP).
//! * {U/P}: Until end of turn, target creature you control becomes a
//!   blue artifact in addition to its other colors and types. Activate
//!   only as a sorcery.
//!
//! The first two lines are pure continuous statics, not expressible on
//! this card class; they are GAP-noted. The {U/P} activation is wired:
//! the artifact-type addition is faithful (additive `AddType`); making
//! the creature blue "in addition to" its colors is a partial because
//! the only color primitive (`SetColor`) replaces rather than adds.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Unctus, Grand Metatect");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(vedalken);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "Other blue creatures you control have '...becomes tapped, draw then discard'" (ability grant).
    // GAP: static "Other artifact creatures you control get +1/+1." (anthem).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U/P}: Until end of turn, target creature you control \
                       becomes a blue artifact in addition to its other colors \
                       and types. Activate only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U/P}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
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
                effect: become_blue_artifact,
            }),
    )
}

fn become_blue_artifact(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        // becomes an artifact in addition to its other types (additive).
        Effect::AddType {
            target: *id,
            types: TypeLine::ARTIFACT.into(),
            duration: Duration::EndOfTurn,
        },
        // becomes blue. NOTE: SetColor replaces rather than adds, so
        // "in addition to its other colors" is a documented fidelity gap.
        Effect::SetColor {
            target: *id,
            colors: ColorSet::blue(),
            duration: Duration::EndOfTurn,
        },
    ]
}
