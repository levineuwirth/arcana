//! Hand of Honor — `{W}{W}` 2/2 Creature — Human Samurai.
//! "Protection from black"
//! "Bushido 1" (Whenever this creature blocks or becomes blocked, it gets
//! +1/+1 until end of turn.)
//!
//! Decomposition:
//! - Keyword line: Bushido 1 → `KeywordAbility::Bushido(1)`.
//!   GAP: Protection from black — the `Protection` keyword is not in the usable
//!   keyword surface (only the parametrized Bushido is), so it is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hand of Honor");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Protection from black not in usable KeywordAbility surface.
        keywords: vec![KeywordAbility::Bushido(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
