//! Boompile — `{4}` artifact (Commander 2016).
//! "{T}: Flip a coin. If you win the flip, destroy all nonland
//! permanents." A coin flip whose win branch sweeps every nonland
//! permanent via `ForEach` over the matching ids.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boompile");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Flip a coin. If you win the flip, destroy all \
                       nonland permanents."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: flip_and_sweep,
            },
        ),
    )
}

fn flip_and_sweep(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
        ctx.controller,
    );
    vec![Effect::FlipCoin {
        player: ctx.controller,
        win: Box::new(Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent {
                target: NULL_OBJECT_ID,
            }),
        }),
        lose: None,
    }]
}
