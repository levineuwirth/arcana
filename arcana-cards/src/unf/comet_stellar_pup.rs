//! Comet, Stellar Pup — `{2}{R}{W}` legendary planeswalker, starting
//! loyalty 5. Subtype Comet; red-white.
//!
//! Loyalty abilities:
//! * `0`: Roll a six-sided die, with per-result branches (squirrel tokens,
//!   graveyard recursion, damage equal to loyalty, extra activations, and
//!   embedded [+N]/[−N] loyalty changes). GAP — there's no six-sided
//!   die-roll primitive (only `FlipCoin`), and the result-driven branching
//!   (including dynamic loyalty deltas and extra-activation grants) is
//!   bespoke. The `0:` shell is declared with no expressible effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Comet, Stellar Pup");
    let comet = reg.interner_mut().intern("Comet");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(comet);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "0: Roll a six-sided die. 1 or 2 — [+2], then create two \
                   1/1 green Squirrel creature tokens with haste until end \
                   of turn. 3 — [−1], then return a card with mana value 2 \
                   or less from your graveyard to your hand. 4 or 5 — Comet \
                   deals damage equal to the number of loyalty counters on \
                   him to a creature or player, then [−2]. 6 — [+1], and you \
                   may activate Comet's loyalty ability two more times this \
                   turn."
                .into(),
            cost: ActivationCost::default(),
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: true,
            activation_zone: arcana_core::registry::ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: zero_roll_gap,
        }),
    )
}

fn zero_roll_gap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: no six-sided die-roll primitive (only FlipCoin); the
    // result-driven branching with dynamic loyalty deltas, damage equal to
    // loyalty, and extra-activation grants is bespoke.
    Vec::new()
}
