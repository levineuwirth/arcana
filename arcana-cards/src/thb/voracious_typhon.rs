//! Voracious Typhon — `{2}{G}{G}` 4/4 green Snake Beast.
//!
//! Escape—`{5}{G}{G}`, Exile four other cards from your graveyard.
//! This creature escapes with three +1/+1 counters on it.
//!
//! Escape is not in the usable keyword surface for this card class and
//! there is no `ActivationCost` field for "exile N cards from your
//! graveyard" as a casting alternative — the whole escape mechanic is a
//! GAP. The card is emitted as faithful vanilla bones.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voracious Typhon");
    let snake = reg.interner_mut().intern("Snake");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Escape ({5}{G}{G}, Exile four other cards from your graveyard;
        // escapes with three +1/+1 counters) — Escape is not an expressible
        // KeywordAbility variant and there is no graveyard-exile alternative
        // casting cost field.
        keywords: vec![] as Vec<KeywordAbility>,
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
