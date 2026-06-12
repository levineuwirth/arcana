//! Pyretic Prankster // Glistening Goremonger — `{1}{R}` Devil creature 2/1.
//! Front face: {3}{B/P}: Transform this creature. Activate only as a sorcery.
//! ({B/P} can be paid with either {B} or 2 life.)
//! Back face: Phyrexian Devil with "When this creature dies, each opponent
//! sacrifices an artifact or creature of their choice."
//!
//! GAP: Hybrid Phyrexian mana cost {B/P} for activated ability not expressible —
//! activated abilities with custom payment shapes not modeled; wired as a triggered
//! ability (ZoneChange to battlefield) approximating ETB instead.
//! GAP: "sorcery speed only" restriction on activated ability not modeled.
//! Back-face dies trigger: wired as `TriggerCondition::SelfDies` face-gated
//! to the back face; "each opponent sacrifices an artifact or creature of
//! their choice" is one `Effect::ChooseNFromZone` per opponent (chooser =
//! that opponent, artifact-or-creature via `with_types_any`, action
//! Sacrifice).

use arcana_core::effects::{Effect, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pyretic Prankster");
    let devil_sub = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(devil_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Glistening Goremonger");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let back_devil_sub = reg.interner_mut().intern("Devil");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(phyrexian_sub);
    back_subtypes.0.insert(back_devil_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: {3}{B/P} activated ability (transform, sorcery-speed) not modeled —
    // OptionalPaymentKind does not support hybrid-Phyrexian mana and activated
    // abilities with custom cost shapes are not in scope.

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Back face: "When this creature dies, each opponent sacrifices
            // an artifact or creature of their choice."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_each_opponent_sacrifices,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

/// Back face: "…each opponent sacrifices an artifact or creature of their
/// choice." One ChooseNFromZone per opponent; the controller constraint is
/// evaluated from the CHOOSER's perspective. Separate top-level effects so
/// each pending choice parks correctly.
fn dies_each_opponent_sacrifices(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
        .controlled_by(ControllerConstraint::You);
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::ChooseNFromZone {
            chooser: opp,
            zone: Zone::Battlefield,
            filter: filter.clone(),
            min: 1,
            max: 1,
            action: PickAction::Sacrifice,
        })
        .collect()
}
