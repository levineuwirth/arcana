//! Knight of Sursi — `{3}{W}` 2/2 Human Knight.
//! Flying; flanking.
//! Suspend 3—{W}.
//!
//! Flying and Flanking are base characteristics. Suspend is not in the usable
//! keyword surface and the suspend cast/time-counter mechanic has no primitive,
//! so it is GAPped.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of Sursi");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Flanking],
        ..Default::default()
    };
    // GAP: "Suspend 3—{W}" — Suspend is not a usable keyword and the
    // suspend cast / time-counter mechanic has no primitive.
    reg.register(CardDefinition::new(name, chars))
}
