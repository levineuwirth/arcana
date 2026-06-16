//! Honored Hydra — `{5}{G}` 6/6 Snake Hydra with Trample.
//! "Embalm {3}{G} ({3}{G}, Exile this card from your graveyard: Create
//! a token that's a copy of it, except it's a white Zombie Snake Hydra
//! with no mana cost. Embalm only as a sorcery.)"
//!
//! Embalm has no KeywordAbility variant, and its graveyard-activated
//! "exile-and-create-a-modified-self-copy" payload is not expressible
//! with the available primitives (no exile-from-graveyard cost field
//! plus a recolored token-copy). It is GAP'd; only Trample is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Honored Hydra");
    let snake = reg.interner_mut().intern("Snake");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Embalm {3}{G} — no Embalm keyword variant; graveyard exile-and-recolored-token-copy not expressible.
    reg.register(CardDefinition::new(name, chars))
}
