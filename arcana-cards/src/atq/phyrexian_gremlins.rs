//! Phyrexian Gremlins — `{2}{B}` 1/1 Phyrexian Gremlin.
//!
//! "You may choose not to untap this creature during your untap step.
//! {T}: Tap target artifact. It doesn't untap during its controller's
//! untap step for as long as this creature remains tapped."
//!
//! The "may choose not to untap during your untap step" static has no
//! primitive in this API surface — GAP'd. The activated ability taps a
//! target artifact and (closest expressible model) adds a Stun counter
//! to skip its next untap; the "for as long as this creature remains
//! tapped" indefinite duration is GAP'd (modeled as a one-step skip).

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Phyrexian Gremlins");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let gremlin = reg.interner_mut().intern("Gremlin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(gremlin);

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

    // GAP: static — "You may choose not to untap this creature during your
    // untap step" (no optional-skip-untap primitive in this API surface).
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tap target artifact. It doesn't untap during its controller's untap step for as long as this creature remains tapped.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::ARTIFACT.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_and_lock,
            }),
    )
}

fn tap_and_lock(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "for as long as this creature remains tapped" indefinite lock —
    // modeled as Tap + a Stun counter (skips the artifact's next untap),
    // the closest expressible "doesn't untap" approximation.
    vec![
        Effect::Tap { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::Stun,
            count: 1,
        },
    ]
}
