//! Jade Orb of Dragonkind — `{2}{G}` artifact (CLB).
//! "{T}: Add {G}. When you spend this mana to cast a Dragon creature
//! spell, it enters with an additional +1/+1 counter on it and gains
//! hexproof until your next turn."
//!
//! Wired: the {T}: Add {G} mana ability. GAP: the spend-tracking rider
//! (counter + hexproof-until-your-next-turn on the Dragon) is not
//! expressible.

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
    let name = reg.interner_mut().intern("Jade Orb of Dragonkind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}: Add {G}. When you spend this mana to cast a Dragon creature spell, it enters with an additional +1/+1 counter on it and gains hexproof until your next turn.".into(),
            cost: ActivationCost::tap_only(),
            target_requirements: Vec::new(),
            is_mana_ability: true,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_green_mana,
        }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "When you spend this mana to cast a Dragon creature spell, it
    // enters with an additional +1/+1 counter on it and gains hexproof
    // until your next turn." — mana-spend riders are not expressible.
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}
