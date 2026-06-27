//! Didgeridoo — `{1}` artifact (Weatherlight, 1997).
//! "{3}: You may put a Minotaur permanent card from your hand onto the
//! battlefield."
//!
//! Modeled faithfully: the {3} activation posts an optional pick over the
//! controller's hand for a Minotaur card and puts it onto the battlefield
//! (`Effect::PutFromHandOntoBattlefield`, which is min-0 so it honours the
//! "you may"). The Minotaur subtype filter is built at resolution.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Didgeridoo");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}: You may put a Minotaur permanent card from your hand onto the battlefield.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: put_minotaur,
        }),
    )
}

fn put_minotaur(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PutFromHandOntoBattlefield {
        player: ctx.controller,
        filter: script::subtype_filter(reg, "Minotaur"),
        tapped: false,
    }]
}
