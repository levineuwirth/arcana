//! Tempt with Bunnies — `{2}{W}` sorcery. "Tempting Offer — Draw a
//! card and create a 1/1 white Rabbit creature token. Then each
//! opponent may draw a card and create a 1/1 white Rabbit creature
//! token. For each opponent who does, you draw a card and you create a
//! 1/1 white Rabbit creature token."
//!
//! The guaranteed first clause (you draw a card and create one 1/1
//! white Rabbit token) is expressed faithfully. The "Tempting offer"
//! remainder — each opponent's optional draw+token and the
//! controller's payoff scaled to how many opponents accepted — has no
//! primitive: there is no per-opponent optional choice that records
//! acceptances and feeds that count back into the controller's draw +
//! token production. That dynamic, choice-driven loop is GAP'd; a fixed
//! stand-in would be materially wrong.

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
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
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
    let rabbit = reg
        .interner()
        .lookup("Rabbit")
        .expect("Rabbit interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    let token = TokenDefinition {
        name: rabbit,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: the "Tempting offer" remainder — each opponent MAY draw a
    // card and create a Rabbit token, and for each opponent who does
    // the controller draws a card and creates a Rabbit token — is not
    // expressible (no per-opponent optional acceptance that feeds its
    // accepted-count back into the controller's payoff). Only the
    // guaranteed first clause is emitted.
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
