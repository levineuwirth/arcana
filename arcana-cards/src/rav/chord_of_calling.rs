//! Chord of Calling — `{X}{G}{G}{G}` convoke instant. "Search your
//! library for a creature card with mana value X or less, put it
//! onto the battlefield, then shuffle." The canonical composition
//! test: X enumeration + convoke cost reduction + tutor-with-
//! dynamic-filter.
//!
//! # Rules references
//!
//! * CR 107.3b — X chosen as the spell is cast; `{X}{G}{G}{G}` at
//!   X=5 is "pay 5 generic + three green."
//! * CR 702.51 — Convoke. Each creature tapped while casting pays
//!   for `{1}` or one mana of that creature's color.
//! * Composition: convoke reduces the generic pips of the expanded
//!   cost (X's `{X}` becomes `Generic(x)`), not the colored pips.
//!   So at X=2 with two convokers, the caster pays only the three
//!   `{G}` pips from the mana pool.
//!
//! # Simplifications
//!
//! * The printed card is a Sorcery + Flash. We model it as an
//!   Instant to sidestep the Flash → instant-speed-typing
//!   interaction. Observable difference is nil for any test that
//!   doesn't rely on Chord *being* a sorcery-typed spell.
//! * "Search your library" enumerates candidates at resolution and
//!   offers a `PickCards` choice, via the existing
//!   [`Effect::TutorToBattlefield`] pipeline. Failure-to-find
//!   (empty candidate set) degrades to "shuffle, resolve with no
//!   effect," which matches the rules.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{CmcCondition, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chord of Calling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        // Convoke is detected by `has_convoke` walking
        // `effective_keywords`; putting it on the base chars is
        // what unlocks the convoke cost-reduction pipeline for
        // this spell.
        keywords: vec![KeywordAbility::Convoke],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a creature card with mana \
                       value X or less, put it onto the battlefield, then \
                       shuffle.".into(),
                // Chord's "target" is the searched card, selected at
                // resolution via `TutorToBattlefield`'s PickCards
                // prompt — not a cast-time TargetRequirement. So the
                // cast has no target_requirements.
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = entry.x_value.unwrap_or(0);
    // Build the search filter dynamically: creature cards with
    // mana value ≤ X. `is_token` is left as None — libraries don't
    // contain tokens, so the token check is redundant here.
    let filter = ObjectFilter {
        types: Some(TypeLine::CREATURE.into()),
        cmc_condition: Some(CmcCondition::Le(x)),
        ..Default::default()
    };
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter,
        tapped: false,
    }]
}
