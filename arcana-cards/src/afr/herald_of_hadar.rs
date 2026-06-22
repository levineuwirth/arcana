//! Herald of Hadar — `{4}{B}` 4/4 Human Warlock (B).
//!
//! * "Circle of Death — {5}{B}: Roll a d20. 1–9 | Each opponent loses
//!   2 life. 10–19 | Each opponent loses 2 life and you gain 2 life.
//!   20 | Each opponent loses 2 life and you gain 2 life. Create two
//!   Treasure tokens."
//!
//! GAP (activated effect): the payoff is a d20 die-roll dispatch. The
//! catalog has no die-roll primitive (only `Effect::FlipCoin`), so the
//! branched outcomes can't be expressed. The {5}{B} activated ability
//! is wired with a GAP'd effect body so the bones + the activation cost
//! land; the "Treasure" Scryfall keyword tag is the 20-result token,
//! not a creature keyword (no keyword line to emit).

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
    let name = reg.interner_mut().intern("Herald of Hadar");
    let human = reg.interner_mut().intern("Human");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Circle of Death — {5}{B}: Roll a d20.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: roll_d20,
            }),
    )
}

fn roll_d20(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: d20 roll dispatch (1–9 / 10–19 / 20 outcomes). No die-roll
    // primitive in the catalog (only FlipCoin).
    Vec::new()
}
