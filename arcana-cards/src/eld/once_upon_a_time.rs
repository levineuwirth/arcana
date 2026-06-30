//! Once Upon a Time — `{2}{G}` instant. "Look at the top five cards of your
//! library. You may put a creature or land card from among them into your hand.
//! Put the rest on the bottom in a random order." (The "If this is the first
//! spell you've cast this game, you may cast it without paying its mana cost"
//! alt-cost is GAP'd — only the {2}{G} mode.)

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Once Upon a Time");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Look at the top five cards of your library. You may put a creature or land card from among them into your hand. Put the rest on the bottom in a random order.".into(),
            target_requirements: Vec::new(),
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
    vec![Effect::DigTopN {
        player: entry.controller,
        count: 5,
        filter: Some(
            ObjectFilter::new()
                .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::LAND)),
        ),
        rest: DigRest::BottomRandom,
    }]
}
