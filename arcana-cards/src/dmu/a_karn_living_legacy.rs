//! A-Karn, Living Legacy — `{4}` Legendary Planeswalker — Karn. Colorless.
//! Starting loyalty 5.
//!
//! +1: Create a tapped Powerstone token.
//! 0: Pay any amount of mana. Look at that many cards from the top of your
//!   library, then put one into your hand and the rest on the bottom in a
//!   random order.
//! −6: You get an emblem with "Tap an untapped artifact you control: This
//!   emblem deals 1 damage to any target."
//!
//! GAP: the +1 Powerstone token cannot be constructed (custom artifact token
//!   with a restricted mana ability) from the demonstrated token APIs; the
//!   ability is declared with its correct cost but its effect is empty.
//! GAP: the 0 ability's "pay any amount of mana, look at that many, keep one"
//!   uses a chosen-X variable payment and a dig effect not expressible from the
//!   demonstrated surface; declared with cost 0, effect empty.
//! GAP: the −6 emblem-creation effect is not expressible; declared with its
//!   correct cost, effect empty.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Karn, Living Legacy");
    let karn = reg.interner_mut().intern("Karn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(karn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // +1: Create a tapped Powerstone token. (Custom token GAP'd.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Create a tapped Powerstone token.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_powerstone,
            })
            // 0: Pay any amount of mana, dig that many, keep one. (GAP'd.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Pay any amount of mana. Look at that many cards from the top of your library, then put one of those cards into your hand and the rest on the bottom of your library in a random order.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_dig,
            })
            // −6: emblem. (GAP'd.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem with \"Tap an untapped artifact you control: This emblem deals 1 damage to any target.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

/// `+1: Create a tapped Powerstone token.`
fn plus_one_powerstone(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot construct a tapped Powerstone artifact token (restricted mana
    // ability) from the demonstrated token APIs.
    Vec::new()
}

/// `0: Pay any amount of mana, look at that many, keep one.`
fn zero_dig(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: chosen-X mana payment + variable dig/keep-one not expressible.
    Vec::new()
}

/// `-6: You get an emblem ...`
fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation is not expressible from the demonstrated surface.
    Vec::new()
}
