//! Tempt with Bunnies — `{2}{W}` sorcery. "Tempting Offer — Draw a card and create a 1/1 white Rabbit
//! creature token. Then each opponent may draw a card and create a 1/1 white Rabbit creature token.
//! For each opponent who does, you draw a card and you create a 1/1 white Rabbit creature token."
//! GAP: Tempting Offer — the opponent-choice branch ("each opponent may") and the follow-up conditional
//! for each accepting opponent are not expressible with the available Effect catalog.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tempt with Bunnies");
    let _rabbit = reg.interner_mut().intern("Rabbit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Tempting Offer — Draw a card and create a 1/1 white Rabbit creature token. Then each opponent may draw a card and create a 1/1 white Rabbit creature token. For each opponent who does, you draw a card and you create a 1/1 white Rabbit creature token.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Tempting Offer — opponent-choice branches not expressible with current Effect catalog
    let rabbit = reg.interner().lookup("Rabbit").expect("Rabbit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    let token = TokenDefinition {
        name: rabbit,
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
