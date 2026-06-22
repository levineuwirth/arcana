//! Mana Leech — `{2}{B}` 1/1 Leech.
//! "You may choose not to untap this creature during your untap step.
//! {T}: Tap target land. It doesn't untap during its controller's untap step
//! for as long as this creature remains tapped."
//!
//! The "may choose not to untap" static is a per-untap-step optional that has
//! no expressible hook here and is GAP'd. The activated ability taps a target
//! land (expressible); the "doesn't untap for as long as ~ remains tapped"
//! continuous lock has no primitive and is GAP'd, so the ability is wired as
//! its best-effort tap.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mana Leech");
    let leech = reg.interner_mut().intern("Leech");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leech);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "You may choose not to untap this creature during your untap
    // step" — no expressible hook for the optional skip-untap.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Tap target land. It doesn't untap during its controller's untap step for as long as this creature remains tapped.".into(),
            cost: ActivationCost {
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tap_target_land,
        }),
    )
}

fn tap_target_land(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "it doesn't untap for as long as this creature remains tapped" — a
    // conditional don't-untap lock has no primitive; best-effort is the tap.
    vec![Effect::Tap { target: *id }]
}
