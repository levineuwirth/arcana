//! Volatile Chimera — `{2}{R}` 3/2 red Elemental Chimera.
//!
//! Rules text:
//! * Before you shuffle your deck to start the game, you may reveal this card
//!   from your deck and exile three or more creature cards you drafted that
//!   aren't in your deck. (a pre-game / draft-time static — not a triggered or
//!   activated ability and not expressible — GAP, no def emitted.)
//! * {1}{R}: Choose a card at random you exiled with cards named Volatile
//!   Chimera. This creature becomes a copy of that card, except it has this
//!   ability.
//!
//! The activated ability's cost ({1}{R}) is expressible, but its effect —
//! becoming a copy of a randomly chosen card from a draft-exile pool keyed by
//! card name — has no representation (no random-exile-pool copy effect). The
//! ability is emitted with its real cost and a GAP'd effect body.

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
    let name = reg.interner_mut().intern("Volatile Chimera");
    let elemental = reg.interner_mut().intern("Elemental");
    let chimera = reg.interner_mut().intern("Chimera");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(chimera);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Before you shuffle your deck to start the game, you may reveal this
    //       card and exile three or more drafted creature cards." — pre-game
    //       draft-time static; not expressible.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Choose a card at random you exiled with cards named Volatile Chimera. This creature becomes a copy of that card, except it has this ability.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: become_random_exiled_copy,
            }),
    )
}

fn become_random_exiled_copy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "become a copy of a card chosen at random from the draft-exile pool
    //       keyed by name" — no random-exile-pool copy effect exists.
    Vec::new()
}
