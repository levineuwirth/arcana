//! Slimefoot's Survey — `{4}{G}` sorcery. "Domain — Search your
//! library for up to two land cards that each have a basic land type,
//! put them onto the battlefield tapped, then shuffle. Look at the top
//! X cards of your library, where X is the number of basic land types
//! among lands you control. Put up to one of them on top of your
//! library and the rest on the bottom of your library in a random
//! order."
//!
//! The search-and-put portion is expressed with two
//! `Effect::TutorToBattlefield` (tapped) over a land filter — the
//! engine shuffles automatically. The "each have a basic land type"
//! refinement is approximated by a land filter (no per-subtype filter
//! variant is available). The second sentence's Domain dig is GAP'd:
//! X = "number of basic land types among lands you control" is not
//! computable with any permitted `script::*` helper (distinct basic
//! land subtype counting is unavailable), and `DigTopN`'s count must be
//! a literal — emitting a fixed count for a dynamic X would be a
//! materially wrong card, so that clause is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slimefoot's Survey");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Search your library for up to two land cards that each have a basic land type, put them onto the battlefield tapped, then shuffle. Look at the top X cards of your library, where X is the number of basic land types among lands you control. Put up to one of them on top of your library and the rest on the bottom of your library in a random order.".into(),
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
    // GAP: the Domain dig clause ("look at the top X cards … X is the
    // number of basic land types among lands you control") cannot be
    // expressed — no script:: helper counts distinct basic land types,
    // and DigTopN's count is a literal.
    vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
    ]
}
