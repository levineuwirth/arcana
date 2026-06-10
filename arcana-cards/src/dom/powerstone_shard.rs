//! Powerstone Shard — `{3}` artifact (Dominaria, 2018).
//! "{T}: Add {C} for each artifact you control named Powerstone
//! Shard." A mana ability with a dynamic amount: the colorless pip
//! count is computed at resolution via `script::count_matching` over
//! artifacts you control with this card's name.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Powerstone Shard");
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
                text: "{T}: Add {C} for each artifact you control named \
                       Powerstone Shard."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_shard_mana,
            },
        ),
    )
}

fn add_shard_mana(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter {
        name: reg.interner().lookup("Powerstone Shard"),
        types: Some(TypeLine::ARTIFACT.into()),
        ..ObjectFilter::default()
    }
    .controlled_by(ControllerConstraint::You);
    let n = script::count_matching(state, &filter, ctx.controller);
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: (0..n)
            .map(|_| ManaUnit::plain(ManaColor::Colorless, ctx.source))
            .collect(),
    }]
}
