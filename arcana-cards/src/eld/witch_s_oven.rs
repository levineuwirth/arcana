//! Witch's Oven — {1} artifact (Throne of Eldraine, 2019).
//! "{T}, Sacrifice a creature: Create a Food token. If the sacrificed
//! creature's toughness was 4 or greater, create two Food tokens
//! instead." Tap + sacrifice-a-creature cost minting a Food token;
//! the toughness-4 upgrade is not expressible (the sacrificed
//! creature's stats are not visible to the resolver).

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Witch's Oven");
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
                text: "{T}, Sacrifice a creature: Create a Food token. If \
                       the sacrificed creature's toughness was 4 or \
                       greater, create two Food tokens instead."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: bake_food,
            },
        ),
    )
}

fn bake_food(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If the sacrificed creature's toughness was 4 or greater,
    // create two Food tokens instead" — the sacrificed cost-payment
    // creature's toughness is not readable from the resolver; always
    // creates one Food.
    vec![Effect::CreateCommodityToken {
        controller: ctx.controller,
        kind: CommodityToken::Food,
        count: 1,
    }]
}
