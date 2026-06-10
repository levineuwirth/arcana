//! Kill Switch — {3} artifact (Mercadian Masques, 1999).
//! "{2}, {T}: Tap all other artifacts. They don't untap during their
//! controllers' untap steps for as long as this artifact remains
//! tapped." Taps every other artifact; the as-long-as-tapped untap
//! lock is not expressible and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kill Switch");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}: Tap all other artifacts. They don't untap \
                       during their controllers' untap steps for as long as \
                       this artifact remains tapped."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: tap_all_other_artifacts,
            },
        ),
    )
}

fn tap_all_other_artifacts(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "They don't untap during their controllers' untap steps for
    // as long as this artifact remains tapped" — a tapped-state-linked
    // untap lock is not expressible; only the tap happens.
    let targets: Vec<_> = script::ids_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        ctx.controller,
    )
    .into_iter()
    .filter(|id| *id != ctx.source)
    .collect();
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::Tap {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
