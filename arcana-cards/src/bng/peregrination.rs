//! Peregrination — `{3}{G}` sorcery. "Search your library for up to two basic
//! land cards with different names, reveal them, put one onto the battlefield
//! tapped and the other into your hand, then shuffle. Scry 1."
//!
//! GAP: TutorToBattlefield does not support `tapped: true` per the catalog
//! shown (wait — the catalog does show `tapped: false` implying tapped is a
//! field, but `tapped: true` would be needed here); also, split two-land
//! search with different names and separate destinations (battlefield vs. hand)
//! is not expressible. Scry 1 alone is insufficient.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Peregrination");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for up to two basic land cards with different names, reveal them, put one onto the battlefield tapped and the other into your hand, then shuffle. Scry 1.".into(),
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
    // GAP: split dual-land-search with different destinations + different-names constraint not expressible
    vec![Effect::Scry { player: entry.controller, count: 1 }]
}
