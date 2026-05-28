//! Holographic Double — `{U}` 1/1 blue Illusion.
//! "{U}, Exile Holographic Double from your hand: Choose a creature card in your hand.
//! Conjure a duplicate of it into your hand."
//! GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name lookup in Effect::execute).
//! GAP: "Exile this card from your hand" — ActivationCost has no exile-from-hand field (only exile_self
//!      for battlefield). Using discard_self as closest approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Holographic Double");
    let illusion = reg.interner_mut().intern("Illusion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}, Exile Holographic Double from your hand: Conjure a duplicate of a creature card in your hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").unwrap(),
                    // GAP: exile-from-hand not in ActivationCost; using discard_self approximation
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: conjure_duplicate,
            }),
    )
}

fn conjure_duplicate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure not modeled (Arena-only mechanic; would need registry-by-name lookup
    // in Effect::execute). Cannot duplicate a chosen card from hand.
    Vec::new()
}
