//! Treasure Vault — artifact land (Adventures in the Forgotten Realms,
//! 2021). "{T}: Add {C}." and "{X}{X}, {T}, Sacrifice this land:
//! Create X Treasure tokens." An artifact land (no mana cost) whose
//! sacrifice activation mints X Treasures via the commodity-token
//! path (X read from the activation's x_value).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Treasure Vault");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::LAND),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{X}, {T}, Sacrifice this land: Create X Treasure \
                       tokens."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{X}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: mint_treasures,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn mint_treasures(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Treasure,
        count: x,
    }]
}
