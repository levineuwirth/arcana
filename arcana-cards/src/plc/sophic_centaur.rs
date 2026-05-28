//! Sophic Centaur — `{3}{G}` 1/1 green Centaur Spellshaper.
//! "{2}{G}{G}, {T}, Discard a card: You gain 2 life for each card in your hand."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sophic Centaur");
    let centaur = reg.interner_mut().intern("Centaur");
    let spellshaper = reg.interner_mut().intern("Spellshaper");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(spellshaper);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}{G}, {T}, Discard a card: You gain 2 life for each card in your hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}{G}").unwrap(),
                    tap: true,
                    discard_self: false,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_life_per_card_in_hand,
            }),
    )
}

fn gain_life_per_card_in_hand(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let hand = script::hand_size(state, ctx.controller);
    let amount = hand * 2;
    vec![Effect::GainLife { player: ctx.controller, amount }]
}
