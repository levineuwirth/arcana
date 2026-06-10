//! Darkwater Egg — {1} artifact (Odyssey, 2001).
//! "{2}, {T}, Sacrifice this artifact: Add {U}{B}. Draw a card."
//! One activation: mana + tap + sacrifice-self, producing one blue
//! and one black mana plus a card. The draw rider means this is not
//! flagged as a mana ability (only-AddMana rule).

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Darkwater Egg");
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
                text: "{2}, {T}, Sacrifice this artifact: Add {U}{B}. Draw \
                       a card."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: crack_egg,
            },
        ),
    )
}

fn crack_egg(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![
                ManaUnit::plain(ManaColor::Blue, ctx.source),
                ManaUnit::plain(ManaColor::Black, ctx.source),
            ],
        },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}
