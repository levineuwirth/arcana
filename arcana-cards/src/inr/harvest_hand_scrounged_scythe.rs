//! Harvest Hand // Scrounged Scythe — `{3}` Artifact Creature — Scarecrow 2/2.
//! When this creature dies, return it to the battlefield transformed under your control.
//! Back face "Scrounged Scythe": Artifact — Equipment.
//! Equipped creature gets +1/+1.
//! As long as equipped creature is a Human, it has menace.
//! Equip {2}.
//!
//! Back-face Equip {2} wired via `with_equip_face_gated({2}, 1)` (face-gated to
//! the back/Equipment face). The "+1/+1 to equipped creature" static installs
//! from a back-face ETB trigger via `ContinuousEffect::attached_pt`, inert while
//! unattached. The front-face "dies → return transformed" trigger is wired
//! (gated to face 0).
//! GAP: "as long as equipped creature is a Human, it has menace" — a conditional
//! attached keyword (keyword only while the host is a Human) has no continuous-
//! effect constructor; left unwired.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back face "Scrounged Scythe": Equip {2} (face-gated to the back/Equipment face).
            .with_equip_face_gated(ManaCost::parse("{2}").expect("valid equip cost"), 1)
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
            .with_trigger_face_gate(1, 0)
            // Back face: install "equipped creature gets +1/+1" on ETB; inert while
            // unattached (incl. while the front-face creature is the battlefield object).
            // GAP: "as long as equipped creature is a Human, it has menace" — a
            // host-type-conditional attached keyword has no continuous-effect
            // constructor; left unwired.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_equip_bonus,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1),
    )
}

fn install_equip_bonus(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
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
