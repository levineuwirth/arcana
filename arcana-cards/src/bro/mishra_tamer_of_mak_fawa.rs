//! Mishra, Tamer of Mak Fawa — `{3}{B}{R}` 4/4 Legendary Human
//! Artificer. Both abilities are static grants (Ward to your
//! permanents; unearth to artifact cards in your graveyard) that the
//! available API cannot express. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mishra, Tamer of Mak Fawa");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: static "Permanents you control have Ward—Sacrifice a
        //   permanent" — granting a non-mana Ward to other permanents
        //   is not expressible.
        // GAP: static "Each artifact card in your graveyard has unearth
        //   {1}{B}{R}" — granting an activated ability to graveyard
        //   cards is not expressible.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
