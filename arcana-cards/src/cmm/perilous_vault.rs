//! Perilous Vault — `{4}` artifact (Magic 2015, 2014).
//! "{5}, {T}, Exile this artifact: Exile all nonland permanents."
//! One activated ability: mana + tap + exile-self, sweeping every
//! nonland permanent into exile.

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
    let name = reg.interner_mut().intern("Perilous Vault");
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
                text: "{5}, {T}, Exile this artifact: Exile all nonland permanents.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    tap: true,
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_all_nonland,
            },
        ),
    )
}

fn exile_all_nonland(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let targets = script::ids_matching(
        state,
        &ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
        ctx.controller,
    );
    vec![Effect::ForEach {
        targets,
        effect: Box::new(Effect::ExilePermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
