//! Tempt with Discovery — `{3}{G}` sorcery. "Tempting offer — Search
//! your library for a land card and put it onto the battlefield. Each
//! opponent may search their library for a land card and put it onto
//! the battlefield. For each opponent who searches a library this way,
//! search your library for a land card and put it onto the
//! battlefield. Then each player who searched a library this way
//! shuffles."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tempt with Discovery");
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
                text: "Tempting offer — Search your library for a land card and put it onto the battlefield. Each opponent may search their library for a land card and put it onto the battlefield. For each opponent who searches a library this way, search your library for a land card and put it onto the battlefield. Then each player who searched a library this way shuffles.".into(),
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
    // Best-effort: the controller's own guaranteed land-to-battlefield search.
    // GAP: the "Tempting offer" mechanic — each opponent MAY also search for a
    // land, and the controller then repeats the search once per opponent who
    // accepted — is not expressible. There is no primitive for offering an
    // optional search to opponents nor for feeding that acceptance count back
    // into a repeated controller search.
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        tapped: false,
    }]
}
