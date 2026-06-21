//! Spike, Tournament Grinder — `{2}{B/P}{B/P}` 1/1 Legendary Human Gamer.
//! "{B/P}{B/P}{B/P}{B/P}: Reveal a card you own from outside the game that
//!  has been banned or restricted in a Constructed format and put it into
//!  your hand." — GAP: the "outside the game" zone and the banned/restricted
//!  predicate are not modeled; no expressible effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spike, Tournament Grinder");
    let human = reg.interner_mut().intern("Human");
    let gamer = reg.interner_mut().intern("Gamer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(gamer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B/P}{B/P}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B/P}{B/P}{B/P}{B/P}: Reveal a card you own from outside the game that has been banned or restricted in a Constructed format and put it into your hand.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B/P}{B/P}{B/P}{B/P}")
                        .expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reveal_from_outside,
            }),
    )
}

fn reveal_from_outside(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Reveal a card you own from outside the game ... put it into your
    // hand" — the outside-the-game zone and the banned/restricted predicate
    // are not modeled.
    Vec::new()
}
