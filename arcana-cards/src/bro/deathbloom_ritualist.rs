//! Deathbloom Ritualist — `{3}{B}{G}` 3/5 black/green Elf Warlock.
//! `{T}: Add X mana of any one color, where X is the number of creature
//! cards in your graveyard.`
//!
//! GAP: "mana of any one color" requires a player-choice for color;
//! ManaUnit::plain only accepts a single fixed ManaColor. Using
//! colorless as placeholder, and the dynamic X amount.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deathbloom Ritualist");
    let elf = reg.interner_mut().intern("Elf");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add X mana of any one color, where X is the number of creature cards in your graveyard.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_x_mana,
            }),
    )
}

fn add_x_mana(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::graveyard_size(state, ctx.controller) as usize;
    if x == 0 {
        return Vec::new();
    }
    // GAP: "any one color" requires player to choose color;
    // adding colorless as placeholder.
    let mana = vec![ManaUnit::plain(ManaColor::Colorless, ctx.source); x];
    vec![Effect::AddMana { player: ctx.controller, mana }]
}
