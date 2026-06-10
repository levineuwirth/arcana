//! Skyshaper — `{2}` artifact.
//! "Sacrifice this artifact: Creatures you control gain flying until end
//! of turn." Sacrifice-self activation; the board-wide grant enumerates
//! your creatures at resolution and applies a per-id `GrantKeyword` via
//! `Effect::ForEach`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skyshaper");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "Sacrifice this artifact: Creatures you control gain \
                       flying until end of turn."
                    .into(),
                cost: ActivationCost {
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_flying_to_all,
            },
        ),
    )
}

fn grant_flying_to_all(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::GrantKeyword {
            target: arcana_core::objects::NULL_OBJECT_ID,
            keyword: KeywordAbility::Flying,
            duration: Duration::EndOfTurn,
        }),
    }]
}
