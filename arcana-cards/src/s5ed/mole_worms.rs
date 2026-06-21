//! Mole Worms — `{2}{B}` 1/1 Worm.
//! "You may choose not to untap this creature during your untap step." (static)
//! "{T}: Tap target land. It doesn't untap during its controller's untap step
//!  for as long as this creature remains tapped."
//!
//! The "may choose not to untap" static is not an expressible primitive (GAP).
//! The activated ability taps a target land — modeled — but the "stays tapped
//! while ~ remains tapped" lock is not expressible, so only the tap is emitted.

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
    let name = reg.interner_mut().intern("Mole Worms");
    let worm = reg.interner_mut().intern("Worm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(worm);

    // GAP: "You may choose not to untap this creature during your untap step."
    // — an untap-restriction static with no expressible primitive.

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

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Tap target land. It doesn't untap during its \
                       controller's untap step for as long as this creature \
                       remains tapped."
                    .into(),
                cost: ActivationCost::tap_only(),
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

fn tap_target_land(
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
    // GAP: "doesn't untap ... for as long as this creature remains tapped" — the
    // persistent untap lock keyed on this creature's tapped state is not
    // expressible; only the one-time tap is modeled.
    vec![Effect::Tap { target: *id }]
}
