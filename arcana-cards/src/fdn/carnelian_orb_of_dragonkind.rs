//! Carnelian Orb of Dragonkind — `{2}{R}` artifact (Commander Legends:
//! Battle for Baldur's Gate, 2022).
//! "{T}: Add {R}. If that mana is spent on a Dragon creature spell, it
//! gains haste until end of turn."
//!
//! GAP: the spend-rider ("if that mana is spent on a Dragon creature
//! spell, it gains haste") is a mana-spend tracking effect with no
//! catalog primitive — only the plain mana ability is wired.

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
    let name = reg.interner_mut().intern("Carnelian Orb of Dragonkind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {R}. If that mana is spent on a Dragon \
                       creature spell, it gains haste until end of turn."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red_mana,
            },
        ),
    )
}

fn add_red_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If that mana is spent on a Dragon creature spell, it gains
    // haste until end of turn" — mana-spend riders are not expressible.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
    }]
}
