//! Fire // Ice — Apocalypse common split card. Fire is `{1}{R}`
//! instant, "deal 2 damage"; Ice is `{1}{U}` instant, "tap target
//! permanent". The seed's touchstone for CR 711 Split cards — two
//! instant halves side by side, either castable from hand via its
//! own cost.
//!
//! # Scope compression
//!
//! Printed oracle for Fire is "Fire deals 2 damage divided as you
//! choose among one or two targets"; this engine's Phase 2 ships
//! Fire as a **single-target** 2-damage bolt, matching the shape of
//! Lightning Bolt / Stomp. Divided-damage target enumeration is a
//! general capability that lands alongside [cards like Shatterskull
//! Smashing and Lightning Helix]; the seed's purpose is Split cast
//! mechanics, not divided damage.
//!
//! Ice is oracle-complete: "Tap target permanent. Draw a card".
//! The two-effect vec is the first seed pair that doesn't pause
//! mid-resolution; see [`preordain`](arcana_cards::m11::preordain)
//! for the sibling test of the pause/resume path via Scry-then-draw.
//!
//! # Rules references
//!
//! * CR 711.1 — Split card. One card with two instant-or-sorcery
//!   halves. Each half has its own name, mana cost, and rules
//!   text.
//! * CR 711.4a — A player casting a split card chooses which half
//!   to cast. The half not chosen has no effect.
//! * CR 711.4b — In zones other than the stack, a split card has
//!   the combined characteristics of both halves. This engine's
//!   Phase 2 does not implement the combined view; queries against
//!   the card in hand or graveyard see only the left (Fire) half.
//!   Documented at [`CardDefinition::with_split_right`].
//! * Fuse (CR 702.102) — not implemented. Fire // Ice predates
//!   Fuse; no current seed requires it.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, ObjectOrPlayer, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    // --- Left half: Fire ({1}{R} instant, 2 damage) -------------------
    let fire_name = reg.interner_mut().intern("Fire");
    let fire_chars = Characteristics {
        name: fire_name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid fire cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let fire_ability = SpellAbilityDef {
        text: "Fire deals 2 damage to any target.".into(),
        target_requirements: vec![TargetRequirement::any_target()],
        modal: None,
        effect: fire_resolve,
    };

    // --- Right half: Ice ({1}{U} instant, tap target permanent) ------
    let ice_name = reg.interner_mut().intern("Ice");
    let ice_chars = Characteristics {
        name: ice_name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid ice cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let ice_ability = SpellAbilityDef {
        text: "Tap target permanent. Draw a card.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(ObjectFilter::permanent()),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: ice_resolve,
    };
    let ice_face = CardFace {
        name: ice_name,
        characteristics: ice_chars,
        spell_ability: Some(ice_ability),
    };

    reg.register(
        CardDefinition::new(fire_name, fire_chars)
            .with_spell_ability(fire_ability)
            .with_split_right(ice_face),
    )
}

fn fire_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: entry.source,
        target: dt,
        amount: 2,
    }]
}

fn ice_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else {
        // Target-permanent filter produces Object targets only;
        // player targets would be a solver bug.
        return Vec::new();
    };
    vec![
        Effect::Tap { target: *id },
        Effect::DrawCards { player: entry.controller, count: 1 },
    ]
}
