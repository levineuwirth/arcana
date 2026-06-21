//! Opposition Agent — `{2}{B}` 3/2 Human Rogue with Flash.
//!
//! "You control your opponents while they're searching their
//! libraries. While an opponent is searching their library, they exile
//! each card they find. You may play those cards for as long as they
//! remain exiled, and you may spend mana as though it were mana of any
//! color to cast them."
//!
//! Flash is a base keyword. The two static search-hijack clauses are
//! continuous replacement effects over an opponent's library search —
//! no primitive in this API surface — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Opposition Agent");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // GAP: static — "You control your opponents while they're searching
    // their libraries." (no search-control replacement primitive).
    // GAP: static — "While an opponent is searching their library, they
    // exile each card they find; you may play those cards…" (no
    // search-hijack / play-from-exile-with-any-color primitive).
    reg.register(CardDefinition::new(name, chars))
}
