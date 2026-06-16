//! Pentarch Paladin — `{2}{W}{W}{W}` 3/3 Human Knight.
//!
//! Flanking
//! "As this creature enters, choose a color." (ETB color choice — not
//! expressible; GAP'd.)
//! "{W}{W}, {T}: Destroy target permanent of the chosen color." (depends
//! on the stored chosen color, which isn't expressible; GAP'd.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pentarch Paladin");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flanking],
        ..Default::default()
    };

    // GAP: ETB "choose a color" + the dependent "{W}{W}, {T}: Destroy
    // target permanent of the chosen color" — no mechanism to store and
    // reference a chosen color in the demonstrated API.

    reg.register(CardDefinition::new(name, chars))
}
