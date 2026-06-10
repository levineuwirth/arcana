//! Scourglass — `{3}{W}{W}` artifact (Shards of Alara).
//! "{T}, Sacrifice this artifact: Destroy all permanents except for
//! artifacts and lands. Activate only during your upkeep." A
//! sacrifice-for-sweeper activation; the upkeep-only timing window is
//! not expressible and is a GAP.

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
    let name = reg.interner_mut().intern("Scourglass");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Sacrifice this artifact: Destroy all permanents except for artifacts and lands. Activate only during your upkeep.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_all_except_artifacts_and_lands,
            },
        ),
    )
}

fn destroy_all_except_artifacts_and_lands(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Activate only during your upkeep" — activation timing windows are
    // not expressible on ActivationCost.
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine(TypeLine::ARTIFACT | TypeLine::LAND));
    let ids = script::ids_matching(state, &filter, ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
