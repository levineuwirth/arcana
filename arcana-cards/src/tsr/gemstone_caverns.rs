//! Gemstone Caverns — Legendary Land (Time Spiral, 2006).
//! "If this card is in your opening hand and you're not the starting
//! player, you may begin the game with Gemstone Caverns on the
//! battlefield with a luck counter on it. If you do, exile a card from
//! your hand." and "{T}: Add {C}. If Gemstone Caverns has a luck counter
//! on it, instead add one mana of any color."
//! GAP: the begin-the-game-on-the-battlefield opening-hand replacement is
//! not expressible. GAP: the luck-counter "instead add one mana of any
//! color" upgrade is not expressible (no counter-conditional mana choice
//! in an activated effect) — only the base "{T}: Add {C}" is wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gemstone Caverns");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Add {C}. If Gemstone Caverns has a luck counter \
                       on it, instead add one mana of any color."
                    .into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            },
        ),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If Gemstone Caverns has a luck counter on it, instead add one
    // mana of any color" — counter-conditional any-color upgrade is not
    // expressible.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}
