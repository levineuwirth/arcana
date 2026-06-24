//! Grimlock, Dinobot Leader // Grimlock, Ferocious King — `{1}{R}{G}{W}` Legendary Artifact Creature
//! — Autobot 4/4 (front) / Legendary Artifact Creature — Dinosaur (back).
//!
//! Front face (Grimlock, Dinobot Leader):
//!   Dinosaurs, Vehicles, and other Transformers creatures you control get +2/+0.
//!   {2}, Convert a Transformers toy you own to its other mode: becomes Grimlock, Ferocious King.
//!
//! Back face (Grimlock, Ferocious King):
//!   Trample
//!   {2}, Convert a Transformers toy you own to its other mode: becomes Grimlock, Dinobot Leader.
//!
//! GAP: "Dinosaurs, Vehicles, and other Transformers creatures you control get +2/+0" —
//!   the type-based static anthem (filtering Autobot creatures or creatures named "Transformers")
//!   is not expressible as a specific ContinuousEffect builder; continuous-effect engine subsystem
//!   for custom type filters deferred.
//! GAP: "Convert a Transformers toy you own to its other mode" — the physical-object cost
//!   is not expressible as an ActivationCost; modeled as a {2} mana cost activation only.
//!
//! The back face's {2} transform-back activated ability IS wired (face-gated to face 1
//! below). The back face has no other (triggered) abilities; Trample is a static keyword
//! carried on the back face's characteristics.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Grimlock, Dinobot Leader");
    let autobot_sub = reg.interner_mut().intern("Autobot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(autobot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Back face: Grimlock, Ferocious King — Legendary Artifact Creature — Dinosaur with Trample
    let back_name = reg.interner_mut().intern("Grimlock, Ferocious King");
    let dinosaur_sub = reg.interner_mut().intern("Dinosaur");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dinosaur_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green() | ColorSet::white(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {2}: transform to Grimlock, Ferocious King.
            // GAP: the "Convert a Transformers toy" physical-object cost is not expressible;
            // modeled as a plain {2} mana activation.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Convert a Transformers toy you own to its other mode: Grimlock, Dinobot Leader becomes Grimlock, Ferocious King.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0),
                effect: transform_to_king,
            })
            // Back face: {2}: transform back to Grimlock, Dinobot Leader.
            // GAP: back-face activated ability face-gated to face 1.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, Convert a Transformers toy you own to its other mode: Grimlock, Ferocious King becomes Grimlock, Dinobot Leader.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: transform_to_leader,
            }),
    )
}

fn transform_to_king(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn transform_to_leader(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
