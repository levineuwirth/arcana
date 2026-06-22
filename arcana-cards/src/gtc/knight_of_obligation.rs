//! Knight of Obligation — `{3}{W}` 2/4 white Human Knight.
//! Vigilance.
//! Extort (Whenever you cast a spell, you may pay {W/B}. If you do, each
//! opponent loses 1 life and you gain that much life.)
//!
//! Abilities:
//!  - Vigilance → `KeywordAbility::Vigilance`.
//!  - Extort: not in the usable keyword surface for this card class → GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: Extort — not an available KeywordAbility variant for this card class
// (would be a "whenever you cast a spell, you may pay {W/B}" optional-payment
// trigger; Extort is not in the usable keyword surface).

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of Obligation");
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
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
