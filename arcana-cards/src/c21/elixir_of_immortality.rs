//! Elixir of Immortality — `{1}` artifact (Magic 2011, 2010).
//! "{2}, {T}: You gain 5 life. Shuffle this artifact and your graveyard
//! into their owner's library." The life gain is wired; GAP: shuffling
//! this artifact and the graveyard into the library is not expressible
//! (no shuffle-zone-into-library effect).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elixir of Immortality");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}: You gain 5 life. Shuffle this artifact \
                       and your graveyard into their owner's library."
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
                is_instant_speed: false,
                face_gate: None,
                effect: elixir,
            },
        ),
    )
}

fn elixir(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Shuffle this artifact and your graveyard into their owner's
    // library" — no shuffle-zone-into-library effect exists.
    vec![Effect::GainLife { player: ctx.controller, amount: 5 }]
}
