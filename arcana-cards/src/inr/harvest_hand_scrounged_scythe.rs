//! Harvest Hand // Scrounged Scythe — `{3}` Artifact Creature — Scarecrow 2/2.
//! When this creature dies, return it to the battlefield transformed under your control.
//! Back face "Scrounged Scythe": Artifact — Equipment.
//! Equipped creature gets +1/+1.
//! As long as equipped creature is a Human, it has menace.
//! Equip {2}.
//!
//! GAP: the entire back-face Equipment (Scrounged Scythe) is unwired. The
//! `CardDefinition::with_equip` builder is the only way to install an Equip
//! ability + the canonical attach resolver, but it hard-codes `face_gate: None`
//! and writes the Equip keyword/ability onto the *front* (base) characteristics —
//! there is no back-face-gated equip entry point, and the attach resolver
//! (`registry::equip_attach`) is private, so a hand-rolled face-gated Equip
//! activated ability cannot reuse it. Wiring this needs an engine change
//! (a face-gate parameter on `with_equip`, or a public attach resolver). The
//! attached statics ("+1/+1 to equipped creature", "menace while equipped
//! creature is a Human") would then install via a transform-trigger that adds
//! `ContinuousEffect::attached_pt` / `attached_keyword`, but are moot until the
//! Equip attach itself is expressible on the back face. The front-face
//! "dies → return transformed" trigger is wired (gated to face 0).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Harvest Hand");
    let scarecrow_sub = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face "Scrounged Scythe": Artifact — Equipment
    let back_name = reg.interner_mut().intern("Scrounged Scythe");
    let equip_sub = reg.interner_mut().intern("Equipment");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(equip_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: back_subtypes,
            // GAP: "+1/+1 to equipped creature" and "menace if Human" static effects not modeled
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // When this creature dies, return it to the battlefield transformed.
            // Front-face ability only (the back face is a noncreature Equipment).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: harvest_hand_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn harvest_hand_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Return to the battlefield transformed under your control.
    // Approximate as ReturnFromGraveyardToBattlefield + Transform.
    // The engine handles the transform flag on entry when the card resolves transformed.
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}
