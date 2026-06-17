//! Soaring Stoneglider — `{2}{W}` 4/3 Elephant Cleric with Flying and
//! Vigilance. Its additional casting cost (exile two cards from your
//! graveyard or pay {1}{W}) is an unexpressible alternative additional
//! cost and is GAP'd; only the keyword line is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soaring Stoneglider");
    let elephant = reg.interner_mut().intern("Elephant");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(cleric);

    // GAP: "As an additional cost to cast this spell, exile two cards from
    // your graveyard or pay {1}{W}" — alternative additional cost not
    // expressible via the documented card-class surface.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
