//! Mystic Meditation — `{3}{U}` sorcery. "Draw three cards. Then
//! discard two cards unless you discard a creature card."
//! GAP: the unless-payment is "discard a CREATURE card" — a TYPED discard.
//! `OptionalPaymentKind::Discard(u32)` discards N cards of the chooser's
//! choice with no type filter, so the typed payment isn't expressible; emit
//! the draw and an unconditional discard-2 as best-effort.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mystic Meditation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards. Then discard two cards unless you discard a creature card.".into(),
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
    // GAP: 'discard 2 unless you discard a CREATURE card' — the unless-payment
    // is a typed discard; OptionalPaymentKind::Discard has no type filter.
    vec![
        Effect::DrawCards { player: entry.controller, count: 3 },
        Effect::Discard { player: entry.controller, count: 2, choice: DiscardChoice::ControllerChooses },
    ]
}
