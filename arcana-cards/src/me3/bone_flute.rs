//! Bone Flute — `{3}` artifact.
//! "{2}, {T}: All creatures get -1/-0 until end of turn."
//! One activated ability: mana + tap cost; the resolver enumerates
//! every creature on the battlefield and applies a -1/-0 pump via
//! `Effect::ForEach`.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
    let name = reg.interner_mut().intern("Bone Flute");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, {T}: All creatures get -1/-0 until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: weaken_all,
        }),
    )
}

fn weaken_all(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: arcana_core::objects::NULL_OBJECT_ID,
            power: -1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
