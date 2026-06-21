//! Noble Banneret — `{2}{W}{W}` 3/3 Human Knight.
//! Draft this card face up.
//! As you draft a creature card, you may reveal it, note its name, then
//!   turn this card face down.
//! As long as you control one or more creatures with a name you noted for
//!   cards named Noble Banneret, this creature and those creatures get
//!   +1/+1 and have lifelink.
//!
//! All non-bones text is draft-time mechanics and a noted-name-gated
//! static anthem — none expressible in this card class. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Noble Banneret");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP (draft): "Draft this card face up." / "As you draft a creature
    //   card, you may reveal it, note its name, then turn this card face
    //   down." — draft-time mechanics, no runtime primitive.
    // GAP (static): noted-name-gated "this creature and those creatures get
    //   +1/+1 and have lifelink."
    reg.register(CardDefinition::new(name, chars))
}
