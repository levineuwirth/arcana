//! Jace, Architect of Thought — `{2}{U}{U}` Legendary Planeswalker —
//! Jace, starting loyalty 4. Blue.
//!
//! Oracle text:
//! * `+1`: Until your next turn, whenever a creature an opponent
//!   controls attacks, it gets -1/-0 until end of turn.
//! * `−2`: Reveal the top three cards of your library. An opponent
//!   separates those cards into two piles. Put one pile into your hand
//!   and the other on the bottom of your library in any order.
//! * `−8`: For each player, search that player's library for a nonland
//!   card and exile it, then that player shuffles. You may cast those
//!   cards without paying their mana costs.
//!
//! # Scope
//!
//! All three abilities are bespoke and not expressible from the
//! demonstrated surface; each is declared with the correct loyalty
//! cost and a GAP'd effect:
//! * `+1` is a floating attack-watcher window debuffing opponent
//!   attackers — the synthesized-trigger/continuous combination isn't
//!   buildable here.
//! * `−2` is a Fact-or-Fiction-style opponent pile split — no
//!   pile-partition primitive.
//! * `−8` is a per-player tutor-exile-then-free-cast — bespoke.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, Architect of Thought");
    let jace = reg.interner_mut().intern("Jace");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(jace);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, whenever a creature an \
                       opponent controls attacks, it gets -1/-0 until end \
                       of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Reveal the top three cards of your library. An \
                       opponent separates those cards into two piles. Put \
                       one pile into your hand and the other on the bottom \
                       of your library in any order.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−8: For each player, search that player's library \
                       for a nonland card and exile it, then that player \
                       shuffles. You may cast those cards without paying \
                       their mana costs.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight,
            }),
    )
}

/// `+1`: floating opponent-attacker debuff window.
fn plus_one(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: until-your-next-turn floating attack trigger applying -1/-0 to
    // opponent attackers is bespoke and not expressible.
    Vec::new()
}

/// `−2`: opponent pile-split fact-or-fiction.
fn minus_two(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: opponent separates revealed cards into two piles — no
    // pile-partition primitive on the surface.
    Vec::new()
}

/// `−8`: per-player tutor-exile + free cast.
fn minus_eight(_s: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: search EACH player's library, exile a nonland, then free-cast
    // those exiled cards — bespoke and not expressible.
    Vec::new()
}
